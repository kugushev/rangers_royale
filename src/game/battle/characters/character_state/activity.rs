mod moving;
pub mod attacking;

use bevy::prelude::*;
use crate::game::battle::characters::character_state::activity::attacking::{AttackState, build_attacking};
use crate::game::battle::characters::character_state::activity::moving::build_moving;

pub(super) fn build_activity(app: &mut App) {
    build_moving(app);
    build_attacking(app);
}

pub enum Activity {
    Idle,
    Moving(Vec2),
    Attacking(AttackState),
}

impl Default for Activity {
    fn default() -> Self { Activity::Idle }
}
