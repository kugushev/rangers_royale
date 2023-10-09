use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::common::moving::MoveCommand;

pub(super) fn build_controller_indirect(app: &mut App) {
    app.add_systems(Update, handle_directives);
}

#[derive(Component, Getters, Default)]
pub struct ControllerIndirect {
    directive: Option<Directive>,
    directive_not_handled: bool,
    pub selected: bool,
}

pub enum Directive {
    MoveTo(Vec2),
}

impl ControllerIndirect {
    pub fn set_directive(&mut self, directive: Directive) {
        self.directive = Some(directive);
        self.directive_not_handled = true;
    }
}

fn handle_directives(mut query: Query<(&mut MoveCommand, &mut ControllerIndirect)>) {
    for (mut move_command, mut controller) in &mut query {
        if let Some(directive) = &controller.directive {
            let completed = match directive {
                Directive::MoveTo(target) => handle_move_to(&mut move_command, *target, controller.directive_not_handled)
            };

            controller.directive_not_handled = false;
            if completed {
                controller.directive = None;
            }
        }
    }
}

fn handle_move_to(move_command: &mut MoveCommand, target: Vec2, first_handle: bool) -> bool {
    if first_handle {
        move_command.set_target(target);
    } else if move_command.target().is_none() {
        // move command finished so we completed the directive
        return true;
    }

    false
}

