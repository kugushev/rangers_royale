pub mod attack_command;
pub mod move_command;

use bevy::prelude::*;
use crate::game::battle::commands::attack_command::build_attack_command;
use crate::game::battle::commands::move_command::build_move_command;

pub(super) fn build_commands(app: &mut App) {
    build_move_command(app);
    build_attack_command(app);
}