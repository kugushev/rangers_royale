use std::collections::hash_map::Iter;
use std::collections::HashMap;
use bevy::prelude::*;
use derive_getters::Getters;

pub(super) fn build_direct_inputs(app: &mut App) {
    app.insert_resource(DirectInputs::default());
    app.add_systems(First, handle_gamepad_input);
}

#[derive(Resource, Default)]
pub struct DirectInputs(HashMap<usize, DirectDeviceInput>);

impl DirectInputs {
    pub fn get_devices(&self) -> Iter<usize, DirectDeviceInput> {
        self.0.iter()
    }

    pub fn get(&self, id: usize) -> Option<&DirectDeviceInput> {
        self.0.get(&id)
    }
}

#[derive(Default, Getters)]
pub struct DirectDeviceInput {
    action_command: Option<()>,
    horizontal: Option<f32>,
    vertical: Option<f32>,
}

fn handle_gamepad_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut inputs: ResMut<DirectInputs>,
) {
    let stick_to_axis = |axis_type, gamepad| {
        let axis = axes.get(GamepadAxis::new(gamepad, axis_type));
        if let Some(value) = axis {
            const DEAD_ZONE: f32 = 0.1;
            if value.abs() > DEAD_ZONE {
                return Some(value);
            }
        }
        None
    };

    let just_pressed = |button, gamepad|
        if button_inputs.just_pressed(GamepadButton::new(gamepad, button))
        {
            Some(())
        } else {
            None
        };

    for gp in gamepads.iter() {
        let input = inputs.0.entry(gp.id).or_insert(default());

        input.horizontal = stick_to_axis(GamepadAxisType::LeftStickX, gp);
        input.vertical = stick_to_axis(GamepadAxisType::LeftStickY, gp);
        input.action_command = just_pressed(GamepadButtonType::South, gp);
    }
}