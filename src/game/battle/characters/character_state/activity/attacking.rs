use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;
use crate::game::battle::characters::attackable::Attackable;
use crate::game::battle::characters::character_animation::{AnimationController, OneShotAnimation};
use crate::game::battle::characters::character_state::activity::Activity;
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::battle::characters::attacker::Attacker;
use crate::game::battle::characters::faction::Faction;
use crate::game::battle::characters::hit_points::HitPoints;
use crate::game::battle::characters::position_tracker::{CharacterDirection, PositionTracker};
use crate::game::battle::value_objects::Damage;
use crate::game::common::obstacle::Obstacle;
use crate::game::utils::Vec3Ex;

pub(super) fn build_attacking(app: &mut App) {
    app.add_systems(Update, handle_player_attack);
}

pub enum AttackState {
    Requested(AttackType),
    Charging(Timer),
    ApplyingDamage,
    Releasing(Timer),
}

pub enum AttackType { Swing }

const ATTACK_ALL_FRAMES: usize = 60;
const ATTACK_CHARGE_FRAMES: usize = 12;
const ATTACK_CUTOFF_FRAMES: usize = 2;
const ATTACK_RELEASE_FRAMES: usize = ATTACK_ALL_FRAMES - ATTACK_CHARGE_FRAMES - ATTACK_CUTOFF_FRAMES;

const ATTACK_TIME_SECONDS: f32 = 0.5;

fn handle_player_attack(mut query: Query<(&mut CharacterState, &mut AnimationController, &GlobalTransform, &mut PositionTracker, &Arms, &Attacker, &Faction, Entity)>,
                        mut targets_q: Query<(&GlobalTransform, &Obstacle, &mut Attackable, &Faction)>,
                        time: Res<Time>) {
    for (mut state, mut animation_controller, transform, position_tracker, arms, attacker, faction, entity) in query.iter_mut() {
        if !state.is_active() {
            animation_controller.suspend_attack();
            continue;
        }

        let attack_state = match state.get_activity_mut() {
            Activity::Attacking(s) => s,
            _ => {
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
                for (target_trans, target_obstacle, target_attackable, target_faction) in &mut targets_q {
                    if !faction.is_rival(target_faction) {
                        continue;
                    }

                    apply_damage(target_trans, target_obstacle, target_attackable, transform.translation().to_vec2(), &position_tracker, arms, attacker, entity);
                }

                *attack_state = AttackState::Releasing(get_attack_time(ATTACK_RELEASE_FRAMES));
            }
            AttackState::Releasing(t) => {
                t.tick(time.delta());
                if t.finished() {
                    state.set_idle();
                }
            }
        }
    }
}

fn apply_damage(transform: &GlobalTransform, obstacle: &Obstacle, mut attackable: Mut<Attackable>,
                character_position: Vec2, position_tracker: &PositionTracker, arms: &Arms, attacker: &Attacker,
                attacker_entity: Entity) {
    let target_position = transform.translation().to_vec2();
    let distance = character_position.distance(target_position);
    let attack_range = arms.attack_distance();

    if distance <= attack_range + *obstacle.radius() {
        let target_in_front = match position_tracker.direction() {
            CharacterDirection::Up => target_position.y > character_position.y,
            CharacterDirection::Down => target_position.y < character_position.y,
            CharacterDirection::Left => target_position.x < character_position.x,
            CharacterDirection::Right => target_position.x > character_position.x,
        };
        if target_in_front {
            let amount = *attacker.damage_amount();
            attackable.apply(Damage::new(amount, attacker_entity));
        }
    }
}

fn get_attack_time(frames: usize) -> Timer {
    let div = frames as f32 / ATTACK_ALL_FRAMES as f32;
    Timer::from_seconds(ATTACK_TIME_SECONDS * div, TimerMode::Once)
}