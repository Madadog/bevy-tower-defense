use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        
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

fn resize_background(
    mut backgrounds: Query<&mut Sprite, With<Background>>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        for mut background in backgrounds.iter_mut() {
            let scale = window.width().min(window.height());
            background.custom_size = Some(Vec2::splat(scale));
        }
    }
}