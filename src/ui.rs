use bevy::prelude::*;

use crate::{components::{Gold, Lives}, stages::CurrentStage};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_ui)
            .add_system(update_ui_gold)
        ;
    }
}

fn build_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let font = asset_server.load("fonts/NotoSans-Regular.ttf");
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "Gold: Not displaying yet.",
            TextStyle {
                font: font.clone(),
                font_size: 50.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..Default::default()
    })
    .insert(ResourceText);
}

#[derive(Component)]
struct ResourceText;

fn update_ui_gold(
    time: Res<Time>,
    //diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<ResourceText>>,
    gold: Res<Gold>,
    lives: Res<Lives>,
    stage: Res<CurrentStage>,
) {
    for mut text in query.iter_mut() {
        let mut fps = 0.0;

        let mut frame_time = time.delta_seconds_f64();

        text.sections[0].value = format!(
            "Gold: {}\nLives: {}\nStage: {}",
            gold.0,
            lives.0,
            stage.index,
        );
    }
}