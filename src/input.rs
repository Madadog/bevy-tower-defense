use bevy::{
    input::{keyboard::KeyboardInput, ElementState, InputSystem},
    prelude::*,
    utils::HashSet,
};

#[derive(Default)]
pub struct PlayerInputPlugin;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct PlayerInputSystem;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Input<PlayerButton>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                player_button_input_system
                    .label(PlayerInputSystem)
                    .after(InputSystem),
            )
            .init_resource::<Axis<PlayerAxis>>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                player_axis_input_system
                    .label(PlayerInputSystem)
                    .after(InputSystem),
            );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAxis {
    MoveX,
    MoveY,
    AimX,
    AimY,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerButton {
    Accept,
    Cancel,
    Shoot,
}

pub fn player_button_input_system(
    mut button_input: ResMut<Input<PlayerButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    gamepad: Res<Input<GamepadButton>>,
) {
    button_input.clear();
    let mut accept_keybinds: HashSet<KeyCode> = HashSet::default();
    accept_keybinds.insert(KeyCode::Space);
    accept_keybinds.insert(KeyCode::Return);
    accept_keybinds.insert(KeyCode::Z);
    let mut accept_gamepad: HashSet<GamepadButtonType> = HashSet::default();
    accept_gamepad.insert(GamepadButtonType::South);

    let mut cancel_keybinds: HashSet<KeyCode> = HashSet::default();
    cancel_keybinds.insert(KeyCode::Escape);
    cancel_keybinds.insert(KeyCode::X);
    let mut cancel_gamepad: HashSet<GamepadButtonType> = HashSet::default();
    cancel_gamepad.insert(GamepadButtonType::East);

    let mut shoot_keybinds: HashSet<KeyCode> = HashSet::default();
    shoot_keybinds.insert(KeyCode::F);
    shoot_keybinds.insert(KeyCode::R);
    let mut shoot_gamepad: HashSet<GamepadButtonType> = HashSet::default();
    shoot_gamepad.insert(GamepadButtonType::RightTrigger);

    let mut update = |keyboard_bindings: HashSet<KeyCode>,
                      gamepad_bindings: HashSet<GamepadButtonType>,
                      button| {
        if keyboard_input.any_just_pressed(keyboard_bindings.clone()) {
            button_input.press(button);
        }
        if keyboard_input.any_just_released(keyboard_bindings) {
            button_input.release(button);
        }

        for gamepad_button in gamepad_bindings {
            let gamepad_button = GamepadButton(Gamepad(0), gamepad_button);
            if gamepad.just_pressed(gamepad_button) {
                button_input.press(button);
            }
            if gamepad.just_released(gamepad_button) {
                button_input.release(button);
            }
        }
    };

    update(accept_keybinds, accept_gamepad, PlayerButton::Accept);
    update(cancel_keybinds, cancel_gamepad, PlayerButton::Cancel);
    update(shoot_keybinds, shoot_gamepad, PlayerButton::Shoot);
}

pub fn player_axis_input_system(
    mut button_input: ResMut<Axis<PlayerAxis>>,
    keyboard_input: Res<Input<KeyCode>>,
    gamepad: Res<Axis<GamepadAxis>>,
) {
    button_input.set(PlayerAxis::MoveX, 0.0);
    button_input.set(PlayerAxis::MoveY, 0.0);

    // Read gamepad
    if let Some(x) = gamepad.get(GamepadAxis(Gamepad(0), GamepadAxisType::LeftStickX)) {
        button_input.set(PlayerAxis::MoveX, x);
    }
    if let Some(y) = gamepad.get(GamepadAxis(Gamepad(0), GamepadAxisType::LeftStickY)) {
        button_input.set(PlayerAxis::MoveY, y);
    }
    if let Some(x) = gamepad.get(GamepadAxis(Gamepad(0), GamepadAxisType::DPadX)) {
        if x.abs() > 0.5 {
            button_input.set(PlayerAxis::MoveX, x);
        }
    }
    if let Some(y) = gamepad.get(GamepadAxis(Gamepad(0), GamepadAxisType::DPadY)) {
        if y.abs() > 0.5 {
            button_input.set(PlayerAxis::MoveY, y);
        }
    }

    // read keyboard
    let up_keybinds = HashSet::from_iter(
        [KeyCode::Up, KeyCode::W]
    );
    let down_keybinds: HashSet<KeyCode> = HashSet::from_iter(
        [KeyCode::Down, KeyCode::S]
    );
    let left_keybinds: HashSet<KeyCode> = HashSet::from_iter(
        [KeyCode::Left, KeyCode::A]
    );
    let right_keybinds: HashSet<KeyCode> = HashSet::from_iter(
        [KeyCode::Right, KeyCode::D]
    );

    if keyboard_input.any_pressed(up_keybinds) {
        button_input.set(PlayerAxis::MoveY, 1.0);
    }
    if keyboard_input.any_pressed(down_keybinds) {
        button_input.set(PlayerAxis::MoveY, -1.0);
    }
    if keyboard_input.any_pressed(right_keybinds) {
        button_input.set(PlayerAxis::MoveX, 1.0);
    }
    if keyboard_input.any_pressed(left_keybinds) {
        button_input.set(PlayerAxis::MoveX, -1.0);
    }
}