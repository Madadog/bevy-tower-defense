use bevy::prelude::*;

use crate::components::Lives;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameOver(false))
            .add_system(lose_system)
            .add_system(game_over);
    }
}

pub struct GameOver(bool);

fn lose_system(
    mut game_over: ResMut<GameOver>,
    lives: Option<Res<Lives>>,
) {
    if let Some(lives) = lives {
        if lives.0 == 0 {
            game_over.0 = true;
        }
    }
}

fn game_over(
    mut commands: Commands,
    entities: Query<Entity>,
    mut game_over: ResMut<GameOver>,
    asset_server: Res<AssetServer>,
) {
    if game_over.0 {
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        let font = asset_server.load("fonts/NotoSans-Regular.ttf");
        commands.spawn_bundle(UiCameraBundle::default());
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "GAME OVER\nYou have lost.",
                        TextStyle {
                            font: font.clone(),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            });
        game_over.0 = false;
    }
}
