use bevy::ecs::query::QueryEntityError;
use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::common::moving::MoveCommand;
use crate::game::common::obstacle::Obstacle;
use crate::game::registry::AttackRange;

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

fn handle_move_to(mut active_q: Query<(&mut MoveCommand, &mut ControllerIndirect)>) {
    for (mut move_command, mut controller) in &mut active_q {
        handle(&mut controller, |d| {
            if let Directive::MoveTo(target, processed) = d {
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

fn handle_attack(mut active_q: Query<(&mut MoveCommand, &mut ControllerIndirect)>) {
    for (mut move_command, mut controller) in &mut active_q {
        handle(&mut controller, |d| {
            if let Directive::Attack(target, range) = d {}
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