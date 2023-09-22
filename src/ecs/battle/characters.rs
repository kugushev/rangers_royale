mod player_characters;
mod animations;

use bevy::prelude::*;
use crate::ecs::battle::characters::player_characters::build_player_characters;
use crate::ecs::scenes::GameScene;

pub(super) fn build_characters(app: &mut App, scene: GameScene) {
    build_player_characters(app, scene);
}



