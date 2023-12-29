use bevy::prelude::*;
use crate::game::battle::characters::character_animation::{AnimationController, OneShotAnimation};
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::utils::AutoResetGate;

pub(super) fn build_disability(app: &mut App) {
    app.add_systems(Update, handle_stun);
}

pub enum Disability {
    Stun(Timer, AutoResetGate),
}

pub const STUN_TIME: f32 = 0.25;

fn handle_stun(mut query: Query<(&mut CharacterState, &mut AnimationController)>, time: Res<Time>) {
    for (mut state, mut animation_controller) in &mut query {
        let (timer, once) = match &mut state.disability {
            Some(Disability::Stun(t, o)) => (t, o),
            _ => continue,
        };

        timer.tick(time.delta());

        if once.enter() {
            animation_controller.request_one_shot(OneShotAnimation::Stun, STUN_TIME);
        }

        if timer.finished() {
            state.disability = None;
        }
    }
}