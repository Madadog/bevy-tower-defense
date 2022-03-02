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
            .add_system(change_tower);
    }
}

fn spawn_indicator(mut commands: Commands) {
    commands
        .spawn()
        .insert(Indicator::new())
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
pub struct Indicator {
    can_build: bool,
    tower: TowerBundle,
}
impl Indicator {
    pub fn new() -> Self {
        Self {
            can_build: true,
            tower: TowerBundle::default(),
        }
    }
}

fn indicator_follow_mouse(
    mouse: Res<CursorPosition>,
    mut indicator: Query<&mut Transform, With<Indicator>>,
) {
    for mut transform in indicator.iter_mut() {
        transform.translation = mouse.0.extend(3.0);
    }
}

fn indicator_overlap(
    mut indicator: Query<(&Transform, &mut Indicator, &mut Sprite)>,
    structures: Query<(&Transform, &StructureRect), Without<Indicator>>,
    path: Res<NavPath>,
) {
    for (indicator_transform, mut indicator, mut sprite) in indicator.iter_mut() {
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
        if overlaps == 0 {
            sprite.color = Color::rgba(0.0, 0.5, 0.0, 0.5);
            indicator.can_build = true;
        } else {
            sprite.color = Color::rgba(0.5, 0.0, 0.0, 0.5);
            indicator.can_build = false;
        }
    }
}

fn indicator_build(
    mut commands: Commands,
    indicator: Query<(&Transform, &Indicator)>,
    mut gold: ResMut<Gold>,
    input: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    for (transform, indicator) in indicator.iter() {
        if (input.just_pressed(KeyCode::T) || mouse.just_pressed(MouseButton::Left))
            && indicator.can_build
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

fn indicator_resize(mut indicator: Query<(&Indicator, &mut Sprite)>) {
    for (indicator, mut sprite) in indicator.iter_mut() {
        sprite.custom_size = Some(indicator.tower.structure_rect.extents);
    }
}

fn change_tower(mut indicator: Query<&mut Indicator>, input: Res<Input<KeyCode>>) {
    for mut indicator in indicator.iter_mut() {
        if input.just_pressed(KeyCode::B) {
            indicator.tower = TowerBundle::big();
        }
        if input.just_pressed(KeyCode::V) {
            indicator.tower = TowerBundle::default();
        }
    }
}

#[derive(Bundle, Clone)]
struct TowerBundle {
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
    fn big() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.3, 0.3, 0.0),
                    custom_size: Some(Vec2::splat(250.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            bullet_generator: BulletGenerator {
                cooldown: Timer::from_seconds(1.5, true),
                bullet_velocity: 10.0,
                bullet_lifespan: 1.0,
                bullet_damage: 10.0,
                bullet_hits: 1,
                ..Default::default()
            },
            aim: Aim::new(500.0),
            structure_rect: StructureRect::from_vec2(Vec2::splat(250.0)),
            gold: Gold(200),
        }
    }
}
