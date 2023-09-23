use bevy::prelude::*;
use crate::ecs::common::animation::build_animation;
use crate::ecs::common::player_input::build_player_input;
use crate::ecs::common::moving::build_moving;

pub mod animation;
pub mod moving;
pub mod player_input;

pub(super) fn build_common(app: &mut App) {
    build_animation(app);
    build_moving(app);
    build_player_input(app);
}
