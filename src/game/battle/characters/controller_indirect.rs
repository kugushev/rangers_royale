use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::battle::characters::character_state::activity::Activity;
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::registry::AttackRange;
use crate::game::utils::Vec3Ex;

pub(super) fn build_controller_indirect(app: &mut App) {
    app.add_systems(Update, handle_move_to)
        .add_systems(Update, handle_attack);
}

#[derive(Component, Getters, Default)]
pub struct ControllerIndirect {
    directive: Option<Directive>,
    pub selected: bool,
}

#[derive(Debug)]
pub enum Directive {
    MoveTo(Vec2, bool),
    Attack(Entity, AttackRange),
}

impl ControllerIndirect {
    pub fn set_directive(&mut self, directive: Directive) {
        self.directive = Some(directive);
    }
}

fn handle_move_to(mut active_q: Query<(&mut CharacterState, &mut ControllerIndirect)>) {
    for (mut character_state, mut controller) in &mut active_q {
        handle(&mut controller, |d| {
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

fn handle_attack(mut active_q: Query<(&mut CharacterState, &mut ControllerIndirect, &GlobalTransform)>,
                 mut passive_q: Query<&GlobalTransform>) {
    for (mut character_state, mut controller, transform) in &mut active_q {
        handle(&mut controller, |d| {
            if let Directive::Attack(target_entity, range) = d {
                let target = match passive_q.get(*target_entity) {
                    Ok(t) => t,
                    _ => { return true; }
                };

                let current_position = transform.translation().to_vec2();
                let target_position = target.translation().to_vec2();

                let attack_distance = range.distance();
                let distance2target = current_position.distance(target_position);

                if distance2target > attack_distance {
                    character_state.set_moving(target_position);
                } else {
                    if !character_state.is_attacking() {
                        character_state.set_attacking();
                    }
                }
            }
            false
        });
    }
}

fn handle(controller: &mut ControllerIndirect, handler: impl FnOnce(&mut Directive) -> bool) {
    let directive = match &mut controller.directive {
        Some(d) => d,
        None => { return; }
    };

    let completed = handler(directive);
    if completed {
        controller.directive = None;
    }
}