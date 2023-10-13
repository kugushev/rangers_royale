use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::battle::commands::attack_command::AttackCommand;
use crate::game::battle::commands::move_command::MoveCommand;
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

fn handle_move_to(mut active_q: Query<(&mut MoveCommand, &mut AttackCommand, &mut ControllerIndirect)>) {
    for (mut move_command, mut attack_command, mut controller) in &mut active_q {
        handle(&mut controller, |d| {
            if let Directive::MoveTo(target, processed) = d {
                attack_command.suspend();

                if !*processed {
                    move_command.set_target(*target);
                    *processed = true;
                } else if move_command.target().is_none() {
                    // move command finished so we completed the directive
                    return true;
                }
            }
            false
        });
    }
}

fn handle_attack(mut active_q: Query<(&mut MoveCommand, &mut AttackCommand, &mut ControllerIndirect, &GlobalTransform)>,
                 mut passive_q: Query<&GlobalTransform>) {
    for (mut move_command, mut attack_command, mut controller, transform) in &mut active_q {
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
                    move_command.set_target(target_position);
                } else {
                    if !attack_command.is_attacking() {
                        attack_command.swing();
                    }
                    move_command.clear();
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