use bevy::prelude::*;
use crate::game::players::actors_inputs::build_actors_inputs;
use crate::game::players::host_cursor::build_lead_cursor;

pub mod host_cursor;
pub mod actors_inputs;

pub(super) fn build_players(app: &mut App) {
    build_lead_cursor(app);
    build_actors_inputs(app);
}