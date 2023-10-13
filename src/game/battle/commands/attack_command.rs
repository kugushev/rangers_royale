use bevy::prelude::*;
use crate::game::battle::characters::character_animation::{AnimationController, OneShotAnimation};

pub(super) fn build_attack_command(app: &mut App) {
    app.add_systems(Update, handle_attack);
}


#[derive(Component, Default)]
pub struct AttackCommand {
    do_attack: Option<AttackState>,
}

impl AttackCommand {
    pub fn swing(&mut self) {
        self.do_attack = Some(AttackState::Requested(AttackType::Swing));
    }

    pub fn is_attacking(&self) -> bool {
        self.do_attack.is_some()
    }

    pub fn suspend(&mut self) {
        self.do_attack = None;
    }
}

enum AttackState {
    Requested(AttackType),
    Charging(Timer),
    ApplyingDamage,
    Releasing(Timer),
}

enum AttackType { Swing }

const ATTACK_ALL_FRAMES: usize = 60;
const ATTACK_CHARGE_FRAMES: usize = 12;
const ATTACK_CUTOFF_FRAMES: usize = 2;
const ATTACK_RELEASE_FRAMES: usize = ATTACK_ALL_FRAMES - ATTACK_CHARGE_FRAMES - ATTACK_CUTOFF_FRAMES;

const ATTACK_TIME_SECONDS: f32 = 1.0;

fn handle_attack(mut query: Query<(&mut AttackCommand, &mut AnimationController)>, time: Res<Time>) {
    for (mut command, mut animation_controller) in query.iter_mut() {
        let attack_state = match &mut command.do_attack {
            Some(s) => s,
            None => {
                animation_controller.suspend_attack();
                continue;
            }
        };

        match attack_state {
            AttackState::Requested(t) => {
                match t {
                    AttackType::Swing => {
                        animation_controller.request_one_shot(OneShotAnimation::Swing, ATTACK_TIME_SECONDS);
                        *attack_state = AttackState::Charging(get_attack_time(ATTACK_CHARGE_FRAMES));
                    }
                }
            }
            AttackState::Charging(t) => {
                t.tick(time.delta());
                if t.finished() {
                    *attack_state = AttackState::ApplyingDamage;
                }
            }
            AttackState::ApplyingDamage => {
                // todo: apply damage to all objects in range
                *attack_state = AttackState::Releasing(get_attack_time(ATTACK_RELEASE_FRAMES));
            }
            AttackState::Releasing(t) => {
                t.tick(time.delta());
                if t.finished() {
                    command.do_attack = None;
                }
            }
        }
    }
}

fn get_attack_time(frames: usize) -> Timer {
    let div = frames as f32 / ATTACK_ALL_FRAMES as f32;
    Timer::from_seconds(ATTACK_TIME_SECONDS * div, TimerMode::Once)
}
