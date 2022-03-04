use bevy::prelude::*;

use crate::{components::{Gold, Lives}, stages::CurrentStage};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(build_ui)
            .add_startup_system(setup)
            .add_system(button_system)
            .add_system(update_ui_gold)
        ;
    }
}

fn build_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    // let font = asset_server.load("fonts/NotoSans-Regular.ttf");

    
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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct TowerButton;

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>, With<TowerButton>),
    >,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());
    let font = asset_server.load("fonts/NotoSans-Regular.ttf");
    //commands.spawn_bundle(UiImage(asset_server.load(circle.png)));
    commands
        .spawn_bundle(screen_fill_node())
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(left_fill(Val::Auto))
                .with_children(|parent| {
                    parent.spawn_bundle(tower_button())
                    .with_children(|parent| {
                        parent.spawn_bundle(tower_text("Default Tower", font.clone()));
                    });
                    parent.spawn_bundle(tower_button())
                    .with_children(|parent| {
                        parent.spawn_bundle(tower_text("Big Tower", font.clone()));
                    });
                    parent.spawn_bundle(tower_button())
                    .with_children(|parent| {
                        parent.spawn_bundle(tower_text("Fast Tower", font.clone()));
                    });
                    parent.spawn_bundle(tower_button())
                    .with_children(|parent| {
                        parent.spawn_bundle(tower_text("Strong Tower", font.clone()));
                    });
                });
            parent
                .spawn_bundle(left_fill(Val::Px(130.0)))
                .with_children(|parent| {
                    parent.spawn_bundle(resource_text(font.clone()))
                    .insert(ResourceText);
                }
            );
            }
        );
    // commands
    //     .spawn_bundle(tower_button())
    //     .with_children(|parent| {
    //         parent.spawn_bundle(TextBundle {
    //             text: Text::with_section(
    //                 "Button",
    //                 TextStyle {
    //                     font: asset_server.load("fonts/NotoSans-Regular.ttf"),
    //                     font_size: 40.0,
    //                     color: Color::rgb(0.9, 0.9, 0.9),
    //                 },
    //                 Default::default(),
    //             ),
    //             ..Default::default()
    //         });
    //     });
}

fn left_fill(height: Val) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(200.0), height),
            border: Rect::all(Val::Px(2.0)),
            padding: Rect {
                top: Val::Px(5.0),
                bottom: Val::Px(5.0),
                left: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
            },
            justify_content: JustifyContent::FlexEnd,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        color: Color::rgb(0.85, 0.85, 0.85).into(),
        ..Default::default()
    }
}

fn screen_fill_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }
}


fn resource_text(font: Handle<Font>) -> TextBundle {
    TextBundle {
        style: Style {
            align_self: AlignSelf::FlexStart,
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "Gold: Not displaying yet.\nLives: Not displaying yet.\nStage: Not displaying yet.",
            TextStyle {
                font,
                font_size: 35.0,
                color: Color::BLACK,
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

#[derive(Bundle)]
struct TowerButtonBundle {
    #[bundle]
    button_bundle: ButtonBundle,
    tower_button: TowerButton,
}

fn tower_button() -> TowerButtonBundle {
    TowerButtonBundle {
        button_bundle: ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(30.0)),
                // center button
                margin: Rect {
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                    left: Val::Auto,
                    right: Val::Auto,
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        },
        tower_button: TowerButton,
    }
}

fn tower_text(name: &str, font: Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            name,
            TextStyle {
                font,
                font_size: 20.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
            Default::default(),
        ),
        ..Default::default()
    }
}