use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::input::direct_inputs::DirectInputs;
use crate::game::utils::Vec3Ex;

pub(in crate::game::battle::characters) fn build_direct(app: &mut App) {
    app.add_systems(PreUpdate, toggle_direct_devices)
        .add_systems(PreUpdate, handle_direct_inputs.after(toggle_direct_devices));
}

#[derive(Component, Default, Getters)]
pub struct ControllerDirect {
    input_device_id: Option<usize>,
}

impl ControllerDirect {
    pub fn active(&self) -> bool { self.input_device_id.is_some() }
}

fn toggle_direct_devices(mut query: Query<&mut ControllerDirect>, direct_inputs: Res<DirectInputs>) {
    // 'cleanup' controllers from unplugged gamepads
    for mut controller in &mut query {
        if let Some(direct_input_id) = controller.input_device_id {
            if direct_inputs.get(direct_input_id).is_none() {
                controller.input_device_id = None;
            }
        }
    }

    'devices: for (&input_id, _) in direct_inputs.get_devices() {
        let mut candidate = None;
        for controller in &mut query {
            if let Some(direct_input_id) = controller.input_device_id {
                if direct_input_id == input_id {
                    continue 'devices;
                }
            } else {
                candidate = Some(controller);
            }
        }

        if let Some(mut controller) = candidate {
            controller.input_device_id = Some(input_id);
        } else {
            warn!("Unable to add new direct device {input_id}")
        }
    }
}

fn handle_direct_inputs(mut query: Query<(&mut CharacterState, &ControllerDirect, &Transform)>, direct_inputs: Res<DirectInputs>) {
    for (mut character_state, controller, transform) in &mut query {
        let device_id = if let Some(x) = controller.input_device_id { x } else { continue; };
        let input = if let Some(x) = direct_inputs.get(device_id) { x } else { continue; };

        const STEP_LENGTH: f32 = 10.;
        let mut position = transform.translation.to_vec2();
        position.x += *input.horizontal() * STEP_LENGTH;
        position.y += *input.vertical() * STEP_LENGTH;
        character_state.set_moving(position);
    }
}