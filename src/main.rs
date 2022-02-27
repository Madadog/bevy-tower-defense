use std::default;

use bevy::prelude::*;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
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
            // transform: todo!(),
            // global_transform: todo!(),
            // texture: todo!(),
            // visibility: todo!(),
            ..Default::default()
        });
}