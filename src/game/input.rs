use bevy::prelude::*;
use crate::game::input::direct_inputs::build_direct_inputs;
use crate::game::input::indirect_input::build_indirect_input;

pub mod indirect_input;
pub mod direct_inputs;

pub(super) fn build_players(app: &mut App) {
    build_indirect_input(app);
    build_direct_inputs(app);
}