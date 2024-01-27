use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::battle::characters::arms::Arms;
use crate::game::battle::characters::character_state::activity::Activity;
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::battle::characters::controllers::direct::{ControllerDirect, is_direct_active};
use crate::game::battle::characters::controllers::indirect::ai::{AiAlgorithm, build_ai};
use crate::game::battle::characters::controllers::indirect::player_input::build_player_input;
use crate::game::battle::characters::position_tracker::PositionTracker;
use crate::game::utils::Vec3Ex;

pub mod player_input;
pub mod ai;

pub(super) fn build_indirect(app: &mut App) {
    app.add_systems(Update, handle_move_to)
        .add_systems(Update, handle_attack)
        .add_systems(Update, handle_idle);

    build_player_input(app);
    build_ai(app);
}

#[derive(Component, Getters)]
pub struct ControllerIndirect {
    directive: Option<Directive>,
    source: DirectiveSource,
}

#[derive(Debug)]
pub enum Directive {
    MoveTo(Vec2, bool),
    Attack(Entity, Arms),
}

pub enum DirectiveSource {
    PlayerInput { selected: bool },
    Ai(AiAlgorithm),
}

impl ControllerIndirect {
    pub fn new(directive_source: DirectiveSource) -> Self {
        Self {
            source: directive_source,
            directive: None,
        }
    }

    pub fn set_directive(&mut self, directive: Directive) {
        self.directive = Some(directive);
    }

    pub fn has_directive(&self) -> bool {
        self.directive.is_some()
    }
}

fn handle_move_to(mut active_q: Query<(&mut CharacterState, &mut ControllerIndirect, Option<&ControllerDirect>)>) {
    for (mut character_state, mut controller, controller_direct) in &mut active_q {
        handle(&mut controller, controller_direct, |d| {
            if let Directive::MoveTo(target, processed) = d {
                if !*processed {
                    character_state.set_moving(*target);
                    *processed = true;
                } else if let Activity::Moving(_) = character_state.get_activity() {
                    // we're still moving
                } else {
                    // move command finished so we completed the directive
                    return true;
                }
            }
            false
        });
    }
}

fn handle_attack(mut active_q: Query<(&mut CharacterState, &mut ControllerIndirect, Option<&ControllerDirect>, &GlobalTransform, &mut PositionTracker)>,
                 passive_q: Query<&GlobalTransform>) {
    for (mut character_state, mut controller, controller_direct, transform, mut position_tracker) in &mut active_q {
        handle(&mut controller, controller_direct, |d| {
            if let Directive::Attack(target_entity, arms) = d {
                let target = match passive_q.get(*target_entity) {
                    Ok(t) => t,
                    _ => return true,
                };

                let current_position = transform.translation().to_vec2();
                let target_position = target.translation().to_vec2();

                const COME_CLOSER_FACTOR: f32 = 0.9;
                let attack_distance = arms.attack_distance();
                let distance2target = current_position.distance(target_position);

                if distance2target > attack_distance * COME_CLOSER_FACTOR {
                    character_state.set_moving(target_position);
                } else {
                    if !character_state.is_attacking() {
                        position_tracker.look_at(target_position, current_position);
                        character_state.set_attacking();
                    }
                }
            }
            false
        });
    }
}

fn handle_idle(mut active_q: Query<(&mut CharacterState, &mut ControllerIndirect, Option<&ControllerDirect>)>) {
    for (mut character_state, mut controller, direct_controller) in &mut active_q {
        if controller.has_directive() {
            continue;
        }

        if is_direct_active(direct_controller) {
            continue;
        }

        character_state.set_idle();
    }
}

fn handle(controller: &mut ControllerIndirect, controller_direct: Option<&ControllerDirect>, handler: impl FnOnce(&mut Directive) -> bool) {
    if is_direct_active(controller_direct) {
        return;
    }

    let directive = match &mut controller.directive {
        Some(d) => d,
        None => { return; }
    };

    let completed = handler(directive);
    if completed {
        controller.directive = None;
    }
}
