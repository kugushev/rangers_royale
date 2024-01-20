use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;
use crate::game::battle::characters::controllers::direct::{ControllerDirect, is_direct_active};
use crate::game::battle::characters::controllers::indirect::{ControllerIndirect, Directive};

pub(super) fn build_retaliation(app: &mut App) {
    app.add_systems(First, forget_offender)
        .add_systems(Last, handle_retaliation);
}

#[derive(Component, Default)]
pub struct Retaliation(Option<Entity>);

impl Retaliation {
    pub fn assign(&mut self, offender: Entity) {
        // it's ok if we reassign the previous offender
        self.0 = Some(offender);
    }

    pub fn take_offender(&mut self) -> Option<Entity> {
        match self.0 {
            None => None,
            Some(entity) => {
                self.0 = None;
                Some(entity)
            }
        }
    }
}

fn forget_offender(mut query: Query<&mut Retaliation>) {
    for mut retaliation in &mut query {
        // let's forget offender to prevent starting attacking someone after several times after awake (if retaliation was not processed due to direct control)
        retaliation.0 = None;
    }
}

fn handle_retaliation(mut query: Query<(&mut ControllerIndirect, &mut Retaliation, &Arms, Option<&ControllerDirect>)>) {
    for (mut indirect, mut retaliation, arms, direct) in &mut query {
        if is_direct_active(direct) { continue; }

        if let Some(Directive::Attack(..)) = indirect.directive() {
            continue;
        }

        let offender = match retaliation.take_offender() {
            Some(entity) => entity,
            None => continue,
        };

        indirect.set_directive(Directive::Attack(offender, *arms));
    }
}