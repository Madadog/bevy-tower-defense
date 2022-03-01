use bevy::prelude::*;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(apply_velocity)
            .add_system(bullet_generator);
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
            velocity: Vec3::new(x, y, z)
        }
    }

    pub const fn from_vec3(velocity: Vec3) -> Self {
        Self {
            velocity,
        }
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
    pub timer: Timer,
}

impl Default for BulletGenerator {
    fn default() -> Self {
        Self { aim: Vec2::new(1.0, 0.0), timer: Timer::from_seconds(1.0, true) }
    }
}

fn bullet_generator(
    mut commands: Commands,
    mut generators: Query<(&mut BulletGenerator, &Transform)>,
    time: Res<Time>,
) {
    for (mut generator, transform) in generators.iter_mut() {
        generator.timer.tick(time.delta());
        if generator.timer.just_finished() {
            commands.spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..Default::default()
                    },
                    transform: (transform.clone().mul_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)))),
                    ..Default::default()
                })
                .insert(Velocity::new(generator.aim.x, generator.aim.y, 0.0));
        }
    }
}