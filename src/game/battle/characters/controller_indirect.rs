use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::battle::characters::controller_indirect::Directive::MoveTo;
use crate::game::common::moving::MoveCommand;
use crate::game::common::obstacle::Obstacle;
use crate::game::registry::AttackRange;

pub(super) fn build_controller_indirect(app: &mut App) {
    app.add_systems(Update, handle_move_to);
    // .add_systems(Update, handle_attack);
}

#[derive(Component, Getters, Default)]
pub struct ControllerIndirect {
    directive: Option<Directive>,
    directive_not_handled: bool,
    pub selected: bool,
}

pub enum Directive {
    MoveTo(Vec2),
    Attack(Entity, AttackRange),
}

impl ControllerIndirect {
    pub fn set_directive(&mut self, directive: Directive) {
        self.directive = Some(directive);
        self.directive_not_handled = true;
    }
}

fn handle_move_to(mut active_q: Query<(&mut MoveCommand, &mut ControllerIndirect)>) {
    for (mut move_command, mut controller) in &mut active_q {
        handle(&mut controller, |d| {
            if let MoveTo(target) = d {
                if controller.directive_not_handled {
                    move_command.set_target(*target);
                } else if move_command.target().is_none() {
                    // move command finished so we completed the directive
                    return true;
                }
            }
            false
        });
    }
}
}

fn handle(controller: &mut ControllerIndirect, handler: impl FnOnce(&Directive) -> bool) {
    let directive = match &controller.directive {
        Some(d) => d,
        None => { return; }
    };

    let completed = handler(directive);
    controller.directive_not_handled = false;
    if completed {
        controller.directive = None;
    }
}


fn handle_move_to1(target: Vec2, first_handle: bool, move_command: &mut MoveCommand) -> bool {
    if first_handle {
        move_command.set_target(target);
    } else if move_command.target().is_none() {
        // move command finished so we completed the directive
        return true;
    }

    false
}


fn handle_attack1(victim: Entity, range: AttackRange, first_handle: bool, passive_q: &Query<(&Obstacle, &GlobalTransform)>, move_command: &mut Mut<MoveCommand>) -> bool {
    let (obstacle_t, transform_t) = match passive_q.get(victim) {
        Ok(t) => t,
        Err(_) => {
            // victim is not found so let's consider this directive as "completed"
            return true;
        }
    };

    let victim_position = transform_t.translation();

    // if victim_position.distance() {}


    // if first_handle {
    //     move_command.set_target(target);
    // } else if move_command.target().is_none() {
    //     // move command finished so we completed the directive
    //     return true;
    // }

    false
}