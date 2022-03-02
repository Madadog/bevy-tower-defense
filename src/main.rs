use std::default;

use bevy::prelude::*;

use crate::input::*;
use crate::components::*;
use crate::background::*;
use crate::pathfinding::*;

mod input;
mod components;
mod background;
mod pathfinding;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerInputPlugin)
        .add_plugin(ComponentsPlugin)
        .add_plugin(BackgroundPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_background)
        .add_system(debug_keys)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn()
        .insert_bundle(
            OrthographicCameraBundle::new_2d()
        )
        .insert(MainCamera);
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            ..Default::default()
        });
    spawn_unit_at(&mut commands, Vec2::new(-500.0, 500.0));
    spawn_tower_at(&mut commands, Vec2::new(0.0, -100.0));
    commands.insert_resource(
        pathfinding::navdata::map1()
    );
}

fn spawn_unit_at(commands: &mut Commands, translation: Vec2) {
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(translation.extend(1.0)),
            ..Default::default()
        })
        .insert(Velocity::new(1.0, 0.0, 0.0))
        .insert(AiUnit)
        .insert(PathFollow::new(0, 1.5));
}

fn spawn_tower_at(commands: &mut Commands, translation: Vec2) {
    commands.spawn()
    .insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..Default::default()
        },
        transform: Transform::from_translation(translation.extend(1.0)),
        ..Default::default()
    })
    .insert(BulletGenerator {
        cooldown: Timer::from_seconds(1.0, true),
        bullet_velocity: 3.0,
        bullet_lifespan: 5.0,
        ..Default::default()
    })
    .insert(Aim::new(250.0));
}

fn debug_keys(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    cursor: Res<CursorPosition>,
) {
    if input.just_pressed(KeyCode::T) {
        spawn_tower_at(&mut commands, cursor.0);
    }
    if input.just_pressed(KeyCode::U) {
        spawn_unit_at(&mut commands, cursor.0);
    }
    if input.just_pressed(KeyCode::Y) {
        println!("Vec2::new({}, {}),", cursor.0.x, cursor.0.y);
    }

}