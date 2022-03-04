use std::time::Duration;

use bevy::prelude::*;

use crate::{pathfinding::follow_path, rectangle::Hitbox};

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .init_resource::<Gold>()
            .insert_resource(Lives(100))
            .add_system(apply_velocity)
            .add_system(bullet_generator)
            .add_system(aim_bullet_generators)
            .add_system(update_cursor_position)
            .add_system(update_lifespan)
            .add_system(despawn_dead)
            .add_system(absorb_bullets)
            .add_system(rotate_bullets);
    }
}

#[derive(Copy, Clone, Debug, Component, Reflect)]
/// Regular old velocity that obeys Newton's first law.
pub struct Velocity {
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

#[derive(Clone, Debug, Component)]
pub struct BulletGenerator {
    pub aim: Vec2,
    pub cooldown: Timer,
    pub shooting: bool,
    pub bullet_velocity: f32,
    pub bullet_lifespan: f32,
    pub bullet_damage: f32,
    pub bullet_hits: u32,
    pub bullet_extents: Vec2,
    pub bullet_texture: Handle<Image>,
}

impl Default for BulletGenerator {
    fn default() -> Self {
        Self {
            aim: Vec2::new(1.0, 0.0),
            cooldown: Timer::from_seconds(1.0, true),
            shooting: true,
            bullet_velocity: 1.0,
            bullet_lifespan: 5.0,
            bullet_damage: 1.0,
            bullet_hits: 1,
            bullet_extents: Vec2::splat(8.0),
            bullet_texture: bevy::render::texture::DEFAULT_IMAGE_HANDLE.typed(),
        }
    }
}

fn bullet_generator(
    mut commands: Commands,
    mut generators: Query<(&mut BulletGenerator, &Transform)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
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
                        custom_size: Some(generator.bullet_extents),
                        ..Default::default()
                    },
                    transform: (transform
                        .clone()
                        .mul_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)))),
                    texture: generator.bullet_texture.clone_weak(),
                    ..Default::default()
                })
                .insert(Velocity::from_vec3(
                    (generator.aim * generator.bullet_velocity).extend(0.0),
                ))
                .insert(Lifespan::new(generator.bullet_lifespan))
                .insert(Bullet::new(
                    generator.bullet_extents,
                    generator.bullet_damage,
                    generator.bullet_hits,
                ));
        }
    }
}

#[derive(Clone, Debug, Component, Reflect)]
pub struct Aim {
    pub radius: f32,
}
impl Aim {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
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
        } else {
            generator.cooldown.set_repeating(false);
            generator.shooting = false;
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

#[derive(Debug, Clone, Component, Reflect)]
/// Damage-absorbing rectangle centered on the owning entity's transform
pub struct Health {
    pub health: f32,
    pub ignore_damage: bool,
    pub ignore_death: bool,
}
impl Health {
    pub fn new(health: f32) -> Self {
        Self {
            health,
            ignore_damage: false,
            ignore_death: false,
        }
    }
    pub fn damage(&mut self, amount: f32) {
        if !self.ignore_damage {
            self.health -= amount;
        }
    }
    pub fn dead(&self) -> bool {
        self.health <= 0.0 && !self.ignore_death
    }
}

pub fn despawn_dead(
    mut commands: Commands,
    mut query: Query<(&Health, Entity, Option<&Gold>)>,
    mut gold_resource: ResMut<Gold>) {
    for (health, entity, gold) in query.iter_mut() {
        if health.dead() {
            if let Some(gold) = gold {
                gold_resource.0 += gold.0;
            }
            commands.entity(entity).despawn_recursive();
        };
    }
}

#[derive(Debug, Clone, Component, Reflect)]
/// Damage-absorbing rectangle centered on the owning entity's transform
pub struct DamageAbsorber {
    pub extents: Vec2,
}
impl DamageAbsorber {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            extents: Vec2::new(x, y),
        }
    }
    pub fn from_vec2(extents: Vec2) -> Self {
        Self { extents }
    }
    pub fn to_hitbox(&self) -> Hitbox {
        Hitbox::with_extents(self.extents)
    }
}

#[derive(Debug, Clone, Component, Reflect)]
/// Damage-dealing rectangle centered on the owning entity's transform.
/// The "hits" field decrements with every target the bullet contacts. When it reaches 0, the bullet despawns.
pub struct Bullet {
    pub extents: Vec2,
    pub damage: f32,
    hits: u32,
    already_hit: Vec<Entity>,
}
impl Bullet {
    pub fn new(extents: Vec2, damage: f32, hits: u32) -> Self {
        Self {
            extents,
            damage,
            hits,
            already_hit: Vec::with_capacity(hits as usize),
        }
    }
    pub fn to_hitbox(&self) -> Hitbox {
        Hitbox::with_extents(self.extents)
    }
}
impl Default for Bullet {
    fn default() -> Self {
        Self {
            extents: Default::default(),
            damage: Default::default(),
            hits: 1,
            already_hit: vec![],
        }
    }
}

pub fn absorb_bullets(
    mut commands: Commands,
    mut targets: Query<(&mut Health, &DamageAbsorber, &Transform, Entity)>,
    mut bullets: Query<(&mut Bullet, &Transform, Entity)>,
) {
    for (mut bullet, transform, bullet_entity) in bullets.iter_mut() {
        let bullet_rect = bullet.to_hitbox().with_translation(transform);
        if bullet.hits == 0 {
            commands.entity(bullet_entity).despawn_recursive();
            break;
        };
        for (mut target, damage_absorber, transform, target_entity) in targets.iter_mut() {
            if bullet.hits == 0 {
                commands.entity(bullet_entity).despawn_recursive();
                break;
            };
            let target_rect = damage_absorber.to_hitbox().with_translation(transform);
            if bullet_rect.touches(&target_rect) && !bullet.already_hit.contains(&target_entity) {
                target.damage(bullet.damage);
                bullet.hits = bullet.hits.saturating_sub(1);
                bullet.already_hit.push(target_entity);
            }
        }
    }
}

pub fn rotate_bullets(
    mut bullets: Query<(&mut Transform, &Velocity), With<Bullet>>,
) {
    for (mut transform, velocity) in bullets.iter_mut() {
        // *transform = transform.with_rotation(Quat::from_axis_angle(Vec3::Z, velocity.velocity.angle_between(Vec3::Y)));
        *transform = transform.with_rotation(Quat::from_rotation_arc(Vec3::Y, velocity.velocity.normalize()));
    }
}

#[derive(Debug, Clone, Component, Reflect, Default)]
pub struct StructureRect {
    pub extents: Vec2,
}
impl StructureRect {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            extents: Vec2::new(x, y),
        }
    }
    pub fn from_vec2(extents: Vec2) -> Self {
        Self { extents }
    }
    pub fn to_hitbox(&self) -> Hitbox {
        Hitbox::with_extents(self.extents)
    }
}

#[derive(Debug, Clone, Component, Reflect, Default)]
/// Gold resource tracks how much the player can spend.
/// On a unit, defines how much gold the player gets when they die.
pub struct Gold(pub u32);
impl Gold {
    pub fn buy(&mut self, cost: u32) -> bool {
        if self.0 >= cost {
            self.0 -= cost;
            true
        } else {
            false
        }
    }
}
fn monitor_gold(gold: Res<Gold>) {
    println!("{:?}", gold);
}

#[derive(Debug, Clone, Component, Reflect, Default)]
/// How many enemies can finish the course before the player loses.
pub struct Lives(pub u32);

fn monitor_lives(lives: Res<Lives>) {
    if lives.0 > 0 {
        println!("{:?}", lives);
    } else {
        println!("You have lost the game.");
    }
}