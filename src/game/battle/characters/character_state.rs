pub mod activity;
pub mod disability;

use bevy::prelude::*;
use crate::game::battle::characters::character_state::activity::{Activity, build_activity};
use crate::game::battle::characters::character_state::activity::attacking::{AttackState, AttackType};
use crate::game::battle::characters::character_state::disability::{build_disability, Disability, STUN_TIME};

pub(super) fn build_character_state(app: &mut App) {
    build_activity(app);
    build_disability(app);
}

#[derive(Component, Default)]
pub struct CharacterState {
    activity: Activity,
    disability: Option<Disability>,
}

impl CharacterState {
    pub fn is_active(&self) -> bool {
        self.disability.is_none()
    }

    pub fn get_activity(&self) -> &Activity {
        &self.activity
    }

    pub fn get_activity_mut(&mut self) -> &mut Activity {
        &mut self.activity
    }

    pub fn set_idle(&mut self) {
        self.activity = Activity::Idle;
    }

    pub fn set_moving(&mut self, target: Vec2) {
        self.activity = Activity::Moving(target);
    }

    pub fn is_attacking(&self) -> bool {
        match self.activity {
            Activity::Attacking(_) => { true }
            _ => { false }
        }
    }

    pub fn set_attacking(&mut self) {
        self.activity = Activity::Attacking(AttackState::Requested(AttackType::Swing));
    }

    pub fn set_stunned(&mut self) {
        self.disability = Some(Disability::Stun(Timer::from_seconds(STUN_TIME, TimerMode::Once), default()));
    }
}