mod player_characters;
pub mod character_animation;
mod position_tracker;
mod character_animations_paths;
mod selection_mark;
mod non_player_characters;
mod controller_indirect;
mod controller_direct;

use bevy::prelude::*;
use crate::game::battle::characters::character_animation::{build_character_animation, CharacterAnimationBundle};
use crate::game::battle::characters::character_animations_paths::CharacterAnimationsPaths;
use crate::game::battle::characters::controller_direct::build_controller_direct;
use crate::game::battle::characters::controller_indirect::build_controller_indirect;
use crate::game::battle::characters::non_player_characters::build_non_player_characters;
use crate::game::battle::characters::player_characters::build_player_characters;
use crate::game::battle::characters::position_tracker::{build_position_tracking, PositionTracker};
use crate::game::battle::characters::selection_mark::build_selection_mark;
use crate::game::battle::commands::attack_command::AttackCommand;
use crate::game::battle::commands::move_command::MoveCommand;
use crate::game::common::cursor_collider::CursorCollider;
use crate::game::common::obstacle::Obstacle;
use crate::game::registry::CHARACTER_RADIUS;

pub(super) fn build_characters(app: &mut App) {
    build_position_tracking(app);
    build_character_animation(app);
    build_player_characters(app);
    build_selection_mark(app);
    build_non_player_characters(app);
    build_controller_indirect(app);
    build_controller_direct(app);
}

#[derive(Component, Default)]
pub struct Character;

#[derive(Bundle)]
pub struct CharacterBundle {
    character: Character,
    position_tracker: PositionTracker,
    animations: CharacterAnimationBundle,
    move_command: MoveCommand,
    attack_command: AttackCommand,
    cursor_collider: CursorCollider,
    obstacle: Obstacle,
}

impl CharacterBundle {
    pub fn new(position: Vec2, paths: &CharacterAnimationsPaths, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> Self {
        Self {
            animations: CharacterAnimationBundle::new(position, paths, asset_server, texture_atlases),
            cursor_collider: CursorCollider::new(Vec2::new(60., 100.), Vec2::new(0., 40.)),
            obstacle: Obstacle::new(CHARACTER_RADIUS),
            character: default(),
            position_tracker: default(),
            move_command: default(),
            attack_command: default()
        }
    }
}
