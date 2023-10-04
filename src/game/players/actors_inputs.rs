use std::collections::hash_map::Iter;
use std::collections::HashMap;
use bevy::prelude::*;
use derive_getters::Getters;

pub(super) fn build_actors_inputs(app: &mut App) {
    app.insert_resource(ActorsInputs::default());
    app.add_systems(First, handle_gamepad_input);
}

#[derive(Resource, Default)]
pub struct ActorsInputs(HashMap<usize, ActorInput>);

impl ActorsInputs {
    pub fn get_actors(&self) -> Iter<usize, ActorInput> {
        self.0.iter()
    }

    pub fn get(&self, id: usize) -> Option<&ActorInput> {
        self.0.get(&id)
    }
}

#[derive(Default, Getters)]
pub struct ActorInput {
    action_command: Option<()>,
    horizontal: f32,
    vertical: f32,
}

fn handle_gamepad_input(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut inputs: ResMut<ActorsInputs>,
) {
    const DEAD_ZONE: f32 = 0.1;
    let stick_to_axis = |axis_type, gamepad| {
        let axis = axes.get(GamepadAxis::new(gamepad, axis_type));
        if let Some(value) = axis {
            if value.abs() > DEAD_ZONE {
                return value;
            }
        }
        0.0
    };

    let just_pressed = |button, gamepad|
        if button_inputs.just_pressed(GamepadButton::new(gamepad, button)) { Some(()) } else { None };

    for gp in gamepads.iter() {
        let input = inputs.0.entry(gp.id).or_insert(default());

        input.horizontal = stick_to_axis(GamepadAxisType::LeftStickX, gp);
        input.vertical = stick_to_axis(GamepadAxisType::LeftStickY, gp);
        input.action_command = just_pressed(GamepadButtonType::South, gp);
    }
}