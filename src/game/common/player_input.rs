use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use derive_getters::Getters;
use crate::game::common::game_cursor::GameCursor;

pub(super) fn build_player_input(app: &mut App) {
    app.insert_resource(PlayerInput::default());
    app.add_systems(First, handle_gamepad_input);
}

#[derive(Resource, Default, Getters)]
pub struct PlayerInput {
    action_command: Option<()>,
    horizontal: f32,
    vertical: f32,
}

fn handle_gamepad_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut player_input: ResMut<PlayerInput>,
) {
    const DEAD_ZONE: f32 = 0.1;
    let stick_to_axis = |axis_type, gamepad| {
        let axis = axes.get(GamepadAxis::new(gamepad, axis_type));
        if let Some(value) = axis {
            if value.abs() > DEAD_ZONE { return value; }
        }
        0.0
    };

    let just_pressed = |button, gamepad| {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, button)) {
            Some(())
        } else { None }
    };
    for gp in gamepads.iter() {
        player_input.horizontal = stick_to_axis(GamepadAxisType::LeftStickX, gp);
        player_input.vertical = stick_to_axis(GamepadAxisType::LeftStickY, gp);
        player_input.action_command = just_pressed(GamepadButtonType::South, gp);

        // if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
        //     info!("{:?} just pressed South", gamepad);
        // } else if button_inputs.just_released(GamepadButton::new(gamepad, GamepadButtonType::South))
        // {
        //     info!("{:?} just released South", gamepad);
        // }
        //
        // let right_trigger = button_axes
        //     .get(GamepadButton::new(
        //         gamepad,
        //         GamepadButtonType::RightTrigger2,
        //     ))
        //     .unwrap();
        // if right_trigger.abs() > 0.01 {
        //     info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        // }
        //
        // let left_stick_x = axes
        //     .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
        //     .unwrap();
        // if left_stick_x.abs() > 0.01 {
        //     info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        // }
    }
}

