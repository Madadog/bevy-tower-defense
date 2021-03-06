use bevy::prelude::*;

use crate::components::*;
use crate::background::*;
use crate::pathfinding::*;
use crate::ui::*;
use crate::stages::*;
use crate::build::*;
use crate::gameover::*;

mod components;
mod background;
mod pathfinding;
mod ui;
mod rectangle;
mod stages;
mod build;
mod gameover;

fn main() {
    println!("Hello, world!");
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Tower Defence".to_string(),
            width: 880.,
            height: 479.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(ComponentsPlugin)
        .add_plugin(BackgroundPlugin)
        .add_plugin(NavigationPlugin)
        .add_plugin(StagePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(BuildPlugin)
        .add_plugin(GameOverPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_background)
        .add_system(debug_keys)
        .run();
}


fn setup(mut commands: Commands) {
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
    commands.insert_resource(
        pathfinding::navdata::map1()
    );
    commands.insert_resource(
        Gold(100)
    );
}

fn spawn_unit_at(commands: &mut Commands, translation: Vec2) {
    commands.spawn()
        .insert_bundle(unitdata::UnitBundle::standard())
        .insert(Transform::from_translation(translation.extend(1.0)));
}

fn debug_keys(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    cursor: Res<CursorPosition>,
    mut gold: ResMut<Gold>,
    mut stages: ResMut<CurrentStage>,
) {
    if input.just_pressed(KeyCode::U) {
        spawn_unit_at(&mut commands, cursor.0);
    }
    if input.just_pressed(KeyCode::Y) {
        println!("Vec2::new({}, {}),", cursor.0.x, cursor.0.y);
    }
    if input.just_pressed(KeyCode::N) || input.just_pressed(KeyCode::Space) {
        stages.start_stage();
        info!("Starting stage {}...", stages.index);
    }
    if input.pressed(KeyCode::G) {
        gold.0 += 1;
    }
}