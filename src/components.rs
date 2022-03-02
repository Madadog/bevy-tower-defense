use std::time::Duration;

use bevy::prelude::*;

use crate::pathfinding::follow_path;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
        .add_system(apply_velocity)
        .add_system(bullet_generator)
        .add_system(aim_bullet_generators)
        .add_system(update_cursor_position)
        .add_system(follow_path)
        .add_system(update_lifespan);
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect)]
pub struct Velocity {
    // Regular old velocity: obeys Newton's first law.
    pub velocity: Vec3,
}
impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            velocity: Vec3::new(x, y, z),
        }
    }

    pub const fn from_vec3(velocity: Vec3) -> Self {
        Self { velocity }
    }
}
impl Velocity {
    pub const ZERO: Velocity = Velocity::from_vec3(Vec3::ZERO);
}
fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation += velocity.velocity;
    }
}

#[derive(Clone, Debug, Component, Reflect)]
pub struct BulletGenerator {
    pub aim: Vec2,
    pub cooldown: Timer,
    pub shooting: bool,
    pub bullet_velocity: f32,
    pub bullet_lifespan: f32,
}

impl Default for BulletGenerator {
    fn default() -> Self {
        Self {
            aim: Vec2::new(1.0, 0.0),
            cooldown: Timer::from_seconds(1.0, true),
            shooting: true,
            bullet_velocity: 1.0,
            bullet_lifespan: 5.0,
        }
    }
}

fn bullet_generator(
    mut commands: Commands,
    mut generators: Query<(&mut BulletGenerator, &Transform)>,
    time: Res<Time>,
) {
    for (mut generator, transform) in generators.iter_mut() {
        generator.cooldown.tick(time.delta());
        if generator.cooldown.finished() && generator.shooting {
            generator.cooldown.reset();
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..Default::default()
                    },
                    transform: (transform
                        .clone()
                        .mul_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)))),
                    ..Default::default()
                })
                .insert(Velocity::from_vec3(
                    (generator.aim * generator.bullet_velocity).extend(0.0)
                ))
                .insert(Lifespan::new(generator.bullet_lifespan));
        }
    }
}

#[derive(Clone, Debug, Component, Reflect)]
pub struct Aim {
    pub radius: f32,
}
impl Aim {
    pub fn new(radius: f32) -> Self { Self {radius} }
}

fn aim_bullet_generators(
    mut generators: Query<(&mut BulletGenerator, &Transform, &Aim)>,
    targets: Query<&Transform, With<AiUnit>>,
) {
    for (mut generator, transform, aim) in generators.iter_mut() {
        let source = transform.translation;
        let target = targets.iter().reduce(|x, y| {
            if x.translation.distance_squared(source) > y.translation.distance_squared(source) {
                y
            } else {
                x
            }
        });
        if let Some(target) = target {
            if aim.radius.powi(2) >= target.translation.distance_squared(source) {
                generator.cooldown.set_repeating(true);
                generator.shooting = true;
                let target = target.translation.truncate();
                let source = source.truncate();
                generator.aim = (target - source).normalize();
            } else {
                generator.cooldown.set_repeating(false);
                generator.shooting = false;
            }
        };
    }
}

#[derive(Clone, Debug, Component, Reflect)]
/// Label for computer-controlled units
pub struct AiUnit;

#[derive(Clone, Debug, Component, Reflect)]
/// Label for the primary game camera
pub struct MainCamera;

/// Mouse location for building towers
#[derive(Debug, Clone, Default)]
pub struct CursorPosition(pub Vec2);

/// "Get mouse window position" example from the unofficial bevy cheat sheet docs (https://bevy-cheatbook.github.io/cookbook/cursor2world.html)
fn update_cursor_position(
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    camera: Query<&Transform, With<MainCamera>>,
    mut cursor_position: ResMut<CursorPosition>,
    
) {
    // get the primary window
    let wnd = windows.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = match camera.get_single() {
            Ok(x) => x,
            Err(_) => return,
        };

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        cursor_position.0 = pos_wld.truncate().truncate();
    }
}

#[derive(Debug, Clone, Component, Reflect)]
/// For entities that only exist for a limited length of time
pub struct Lifespan(Timer);

impl Lifespan {
    pub fn new(time: f32) -> Self {
        Lifespan(Timer::from_seconds(time, false))
    }
    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }
    pub fn finished(&self) -> bool {
        self.0.finished()
    }
}

pub fn update_lifespan(
    mut commands: Commands,
    mut query: Query<(&mut Lifespan, Entity)>,
    time: Res<Time>,
) {
    for (mut life, entity) in query.iter_mut() {
        life.tick(time.delta());
        if life.finished() {
            commands.entity(entity).despawn_recursive();
        };
    }
}