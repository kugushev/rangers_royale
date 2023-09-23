mod player_characters;
mod character_animation;
mod position_tracking;

use bevy::prelude::*;
use crate::ecs::battle::characters::character_animation::build_character_animation;
use crate::ecs::battle::characters::player_characters::build_player_characters;
use crate::ecs::battle::characters::position_tracking::{build_position_tracking, PositionTracker};
use crate::ecs::scenes::GameScene;

pub(super) fn build_characters(app: &mut App, scene: GameScene) {
    build_position_tracking(app);
    build_character_animation(app, scene);
    build_player_characters(app, scene);
}

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    character: Character,
    position_tracker: PositionTracker,
}

#[derive(Component, Default)]
pub struct Character;


