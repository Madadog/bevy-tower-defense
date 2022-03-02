use bevy::prelude::*;

use crate::{components::{CursorPosition, StructureRect, Gold, Aim, BulletGenerator}, rectangle::Hitbox, pathfinding::NavPath};

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_indicator)
            .add_system(indicator_overlap)
            .add_system(indicator_follow_mouse)
            .add_system(indicator_build);
    }
}

fn spawn_indicator(
    mut commands: Commands,
) {
    commands.spawn()
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


#[derive(Debug, Clone, Component, Reflect, Default)]
/// Marker for entity where building occurs
pub struct Indicator {
    can_build: bool,
}
impl Indicator {
    pub fn new() -> Self {
        Self {
            can_build: true,
        }
    }
}

fn indicator_follow_mouse(
    mouse: Res<CursorPosition>,
    mut indicator: Query<&mut Transform, With<Indicator>>
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
        let indicator_rect = Hitbox::with_extents(Vec2::splat(32.0)).with_translation(indicator_transform);
        let mut overlaps = structures.iter().filter(|(structure_transform, rect)| {
            // rect.to_hitbox().with_translation(structure_transform).point_touches(&indicator_transform.translation.truncate())
            rect.to_hitbox().with_translation(structure_transform).touches(&indicator_rect)
        }).count();
        overlaps += path.iter().filter(|x| {
            x.distance_squared(indicator_transform.translation.truncate()) < 32.0_f32.powi(2)
        }).count();
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
        && indicator.can_build && gold.buy(100) {
            crate::spawn_tower_at(&mut commands, transform.translation.truncate());
        }
    }
}


struct TowerBundle {
    sprite_bundle: SpriteBundle,
    bullet_generator: BulletGenerator,
    aim: Aim,
    structure_rect: StructureRect,
}