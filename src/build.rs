use bevy::prelude::*;

use crate::{
    components::{Aim, BulletGenerator, CursorPosition, Gold, StructureRect},
    pathfinding::NavPath,
    rectangle::Hitbox,
};

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_indicator)
            .add_system(indicator_overlap)
            .add_system(indicator_follow_mouse)
            .add_system(indicator_build)
            .add_system(indicator_resize)
            .add_system(indicator_recolour)
            .add_system(change_tower);
    }
}

fn spawn_indicator(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(BuildIndicator::new(&asset_server))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.5, 0.0, 0.5),
                custom_size: Some(Vec2::splat(32.0)),
                ..Default::default()
            },
            ..Default::default()
        });
}

#[derive(Clone, Component, Default)]
/// Marker for entity where building occurs
pub struct BuildIndicator {
    overlapping: bool,
    out_of_bounds: bool,
    //active: bool,
    pub tower: TowerBundle,
}
impl BuildIndicator {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            overlapping: false,
            out_of_bounds: false,
            tower: TowerBundle::dart(asset_server),
        }
    }
    pub fn can_build(&self) -> bool {
        !self.overlapping && !self.out_of_bounds// && self.active
    }
}

fn indicator_follow_mouse(
    mouse: Res<CursorPosition>,
    mut indicator: Query<(&mut Transform, &mut BuildIndicator)>,
) {
    for (mut transform, mut indicator) in indicator.iter_mut() {
        transform.translation = mouse.0.clamp(Vec2::splat(-512.0), Vec2::splat(512.0)).extend(3.0);
        if mouse.0.x > -512.0 && mouse.0.x < 512.0
        && mouse.0.y > -512.0 && mouse.0.y < 512.0 {
            indicator.out_of_bounds = false;
        } else {
            indicator.out_of_bounds = true;
        }
    }
}

fn indicator_overlap(
    mut indicator: Query<(&Transform, &mut BuildIndicator)>,
    structures: Query<(&Transform, &StructureRect), Without<BuildIndicator>>,
    path: Res<NavPath>,
    gold: Res<Gold>,
) {
    for (indicator_transform, mut indicator) in indicator.iter_mut() {
        let indicator_rect = Hitbox::with_extents(indicator.tower.structure_rect.extents)
            .with_translation(indicator_transform);
        let mut overlaps = structures
            .iter()
            .filter(|(structure_transform, rect)| {
                rect.to_hitbox()
                    .with_translation(structure_transform)
                    .touches(&indicator_rect)
            })
            .count();
        overlaps += path
            .iter()
            .filter(|x| {
                x.distance_squared(indicator_transform.translation.truncate())
                    < (indicator.tower.structure_rect.extents.x / 2.0 + 20.0).powi(2)
            })
            .count();
        if overlaps == 0 && indicator.tower.gold.0 <= gold.0 {
            indicator.overlapping = false;
        } else {
            indicator.overlapping = true;
        }
    }
}

fn indicator_build(
    mut commands: Commands,
    indicator: Query<(&Transform, &BuildIndicator)>,
    mut gold: ResMut<Gold>,
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    for (transform, indicator) in indicator.iter() {
        if (input.just_pressed(KeyCode::T) || mouse.just_pressed(MouseButton::Left))
            && indicator.can_build()
            && gold.buy(indicator.tower.gold.0)
        {
            let mut translation = transform.translation;
            translation.z = 1.0;
            commands
                .spawn_bundle(indicator.tower.clone())
                .insert(Transform::from_translation(translation));
        }
    }
}

fn indicator_resize(mut indicator: Query<(&BuildIndicator, &mut Sprite)>) {
    for (indicator, mut sprite) in indicator.iter_mut() {
        sprite.custom_size = Some(indicator.tower.structure_rect.extents);
    }
}

fn indicator_recolour(mut indicator: Query<(&BuildIndicator, &mut Sprite)>) {
    for (indicator, mut sprite) in indicator.iter_mut() {
        if indicator.can_build() {
            sprite.color = Color::rgba(0.0, 0.5, 0.0, 0.5);
        } else {
            sprite.color = Color::rgba(0.5, 0.0, 0.0, 0.5);
        }
    }
}

fn change_tower(mut indicator: Query<&mut BuildIndicator>, input: Res<Input<KeyCode>>, asset_server: Res<AssetServer>) {
    for mut indicator in indicator.iter_mut() {
        if input.just_pressed(KeyCode::B) {
            indicator.tower = TowerBundle::big(&asset_server);
        }
        if input.just_pressed(KeyCode::V) {
            indicator.tower = TowerBundle::dart(&asset_server);
        }
        if input.just_pressed(KeyCode::C) {
            indicator.tower = TowerBundle::fast();
        }
        if input.just_pressed(KeyCode::X) {
            indicator.tower = TowerBundle::strong();
        }
    }
}

#[derive(Bundle, Clone)]
pub struct TowerBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    bullet_generator: BulletGenerator,
    aim: Aim,
    structure_rect: StructureRect,
    gold: Gold,
}
impl Default for TowerBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            bullet_generator: BulletGenerator {
                cooldown: Timer::from_seconds(0.6, true),
                bullet_velocity: 6.0,
                bullet_lifespan: 1.0,
                bullet_damage: 1.0,
                bullet_hits: 1,
                ..Default::default()
            },
            aim: Aim::new(250.0),
            structure_rect: StructureRect::from_vec2(Vec2::splat(32.0)),
            gold: Gold(100),
        }
    }
}
impl TowerBundle {
    pub fn dart(asset_server: &AssetServer) -> Self {
        Self {
            bullet_generator: BulletGenerator {
                bullet_texture: asset_server.load("arrow.png"),
                cooldown: Timer::from_seconds(0.6, true),
                bullet_velocity: 6.0,
                bullet_lifespan: 1.0,
                bullet_damage: 1.0,
                bullet_hits: 1,
                bullet_extents: Vec2::splat(16.0),
                ..Default::default()
            },
            ..Default::default()
        }
    }
    pub fn big(asset_server: &AssetServer) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.3, 0.3, 0.0),
                    custom_size: Some(Vec2::splat(150.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            bullet_generator: BulletGenerator {
                cooldown: Timer::from_seconds(1.5, true),
                bullet_velocity: 10.0,
                bullet_lifespan: 1.0,
                bullet_damage: 5.0,
                bullet_hits: 3,
                bullet_extents: Vec2::splat(64.0),
                bullet_texture: asset_server.load("arrow.png"),
                ..Default::default()
            },
            aim: Aim::new(500.0),
            structure_rect: StructureRect::from_vec2(Vec2::splat(150.0)),
            gold: Gold(200),
        }
    }
    pub fn fast() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.3, 1.0),
                    custom_size: Some(Vec2::splat(36.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            bullet_generator: BulletGenerator {
                cooldown: Timer::from_seconds(0.1, true),
                bullet_velocity: 10.0,
                bullet_lifespan: 1.0,
                bullet_damage: 1.0,
                bullet_hits: 1,
                ..Default::default()
            },
            aim: Aim::new(300.0),
            structure_rect: StructureRect::from_vec2(Vec2::splat(36.0)),
            gold: Gold(800),
        }
    }
    pub fn strong() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.9, 0.1, 0.3),
                    custom_size: Some(Vec2::splat(48.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            bullet_generator: BulletGenerator {
                cooldown: Timer::from_seconds(0.1, true),
                bullet_velocity: 27.0,
                bullet_lifespan: 1.0,
                bullet_damage: 1.0,
                bullet_hits: 3,
                bullet_extents: Vec2::splat(32.0),
                ..Default::default()
            },
            aim: Aim::new(800.0),
            structure_rect: StructureRect::from_vec2(Vec2::splat(48.0)),
            gold: Gold(2000),
        }
    }
}
