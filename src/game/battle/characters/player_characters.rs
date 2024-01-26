use std::ops::Deref;
use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;
use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::controllers::direct::ControllerDirect;
use crate::game::battle::characters::controllers::indirect::{ControllerIndirect, DirectiveSource};
use crate::game::battle::characters::controllers::indirect::ai::AiAlgorithm;
use crate::game::battle::characters::faction::Faction;
use crate::game::battle::characters::selection_mark::SelectionMarkBundle;
use crate::game::game_mode::GameMode;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_player_characters(app: &mut App) {
}

#[derive(Component)]
pub struct PlayerCharacter;