use std::default;

use bevy::prelude::*;

use crate::input::*;
use crate::components::*;

mod input;
mod components;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerInputPlugin)
        .add_plugin(ComponentsPlugin)
        .add_startup_system(setup)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn()
        .insert_bundle(
            OrthographicCameraBundle::new_2d()
        );
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            ..Default::default()
        });
    spawn_unit(&mut commands);
    spawn_tower(&mut commands);
}

fn spawn_unit(commands: &mut Commands) {
    commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
            ..Default::default()
        })
        .insert(Velocity::new(0.0, -1.0, 0.0));
    }
    
    fn spawn_tower(mut commands: &mut Commands) {
        commands.spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(128.0, 128.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
            ..Default::default()
        })
        .insert(BulletGenerator::default());
}

