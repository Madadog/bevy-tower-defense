use bevy::prelude::*;

use crate::components::MainCamera;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(resize_camera);
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Background;

pub fn spawn_background(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("debugmap.png");
    commands.spawn()
    .insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            ..Default::default()
        },
        texture,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    })
    .insert(Background);
}

fn resize_camera(
    mut camera: Query<&mut Transform, With<MainCamera>>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        for mut camera in camera.iter_mut() {
            let scale = window.width().min(window.height()) / 1024.0;
            camera.scale = Vec2::splat(1.0 / scale).extend(1.0);
        }
    }
}