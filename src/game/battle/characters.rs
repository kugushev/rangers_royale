mod player_characters;
mod character_animation;
mod position_tracker;
mod character_animations_paths;

use bevy::prelude::*;
use crate::game::battle::characters::character_animation::build_character_animation;
use crate::game::battle::characters::player_characters::build_player_characters;
use crate::game::battle::characters::position_tracker::{build_position_tracking, PositionTracker};
use crate::game::game_mode::GameMode;

pub(super) fn build_characters(app: &mut App) {
    build_position_tracking(app);
    build_character_animation(app);
    build_player_characters(app);
}

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    character: Character,
    position_tracker: PositionTracker,
}

#[derive(Component, Default)]
pub struct Character;
