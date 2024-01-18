use bevy::prelude::*;
use crate::game::battle::characters::character_animation::{AnimationController, OneShotAnimation};
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::utils::AutoResetGate;

pub(super) fn build_disability(app: &mut App) {
    app.add_systems(Update, handle_stun)
        .add_systems(Update, handle_death);
}

pub enum Disability {
    Stun(Timer, AutoResetGate),
    Death(Timer, AutoResetGate)
}

pub const STUN_TIME: f32 = 0.25;
pub const DEATH_TIME: f32 = 1.0;


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

fn handle_death(mut query: Query<(Entity, &mut CharacterState, &mut AnimationController)>, time: Res<Time>, mut commands: Commands) {
    for (entity, mut state, mut animation_controller) in &mut query {
        let (timer, once) = match &mut state.disability {
            Some(Disability::Death(t, o)) => (t, o),
            _ => continue,
        };

        timer.tick(time.delta());

        if once.enter() {
            println!("Animate death");
            animation_controller.request_one_shot(OneShotAnimation::Death, DEATH_TIME);
        }

        if timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}