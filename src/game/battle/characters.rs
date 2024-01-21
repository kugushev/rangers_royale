mod player_characters;
pub mod character_animation;
mod position_tracker;
mod character_animations_paths;
mod selection_mark;
mod non_player_characters;
pub mod character_state;
pub mod arms;
mod hit_points;
mod attacker;
mod faction;
mod attackable;
mod retaliation;
mod controllers;
pub mod evade_strategy;

use bevy::prelude::*;
use crate::game::battle::characters::arms::build_arms;
use crate::game::battle::characters::attackable::{Attackable, build_attackable};
use crate::game::battle::characters::character_animation::{build_character_animation, CharacterAnimationBundle};
use crate::game::battle::characters::character_animations_paths::CharacterAnimationsPaths;
use crate::game::battle::characters::character_state::{build_character_state, CharacterState};
use crate::game::battle::characters::attacker::{Attacker, build_attacker};
use crate::game::battle::characters::controllers::build_controllers;
use crate::game::battle::characters::faction::Faction;
use crate::game::battle::characters::hit_points::{build_hit_points, HitPoints};
use crate::game::battle::characters::non_player_characters::build_non_player_characters;
use crate::game::battle::characters::evade_strategy::{build_evade_strategy, EvadeStrategy};
use crate::game::battle::characters::player_characters::build_player_characters;
use crate::game::battle::characters::position_tracker::{build_position_tracking, PositionTracker};
use crate::game::battle::characters::retaliation::{build_retaliation, Retaliation};
use crate::game::battle::characters::selection_mark::build_selection_mark;
use crate::game::common::cursor_collider::CursorCollider;
use crate::game::common::obstacle::Obstacle;
use crate::game::registry::{CHARACTER_RADIUS, CharacterOrigin};

pub(super) fn build_characters(app: &mut App) {
    build_position_tracking(app);
    build_character_animation(app);
    build_player_characters(app);
    build_selection_mark(app);
    build_non_player_characters(app);
    build_character_state(app);
    build_arms(app);
    build_hit_points(app);
    build_attacker(app);
    build_attackable(app);
    build_retaliation(app);
    build_controllers(app);
    build_evade_strategy(app);
}

#[derive(Component)]
pub struct Character(CharacterOrigin);

impl Character {
    pub fn origin(&self) -> CharacterOrigin { self.0 }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    character: Character,
    faction: Faction,
    position_tracker: PositionTracker,
    animations: CharacterAnimationBundle,
    character_state: CharacterState,
    cursor_collider: CursorCollider,
    obstacle: Obstacle,
    hit_points: HitPoints,
    attacker: Attacker,
    attackable: Attackable,
    retaliation: Retaliation,
    evade_strategy: EvadeStrategy
}

impl CharacterBundle {
    pub fn new(origin: CharacterOrigin, faction: Faction, position: Vec2, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) -> Self {
        let paths = CharacterAnimationsPaths::find(origin);
        Self {
            character: Character(origin),
            faction,
            animations: CharacterAnimationBundle::new(position, paths, asset_server, texture_atlases),
            cursor_collider: CursorCollider::new(Vec2::new(60., 100.), Vec2::new(0., 40.)),
            obstacle: Obstacle::new(CHARACTER_RADIUS),
            position_tracker: default(),
            character_state: default(),
            hit_points: default(),
            attacker: default(),
            attackable: default(),
            retaliation: default(),
            evade_strategy: EvadeStrategy::new()
        }
    }
}
