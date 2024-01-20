use bevy::prelude::*;
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::battle::characters::hit_points::HitPoints;
use crate::game::battle::characters::retaliation::Retaliation;
use crate::game::battle::value_objects::Damage;

pub(super) fn build_attackable(app: &mut App) {
    app.add_systems(PostUpdate, handle_incoming_damage);
}

#[derive(Component, Default)]
pub struct Attackable(Vec<Damage>);

impl Attackable {
    pub fn apply(&mut self, damage: Damage) {
        self.0.push(damage);
    }
}

fn handle_incoming_damage(mut query: Query<(&mut Attackable, &mut HitPoints, &mut CharacterState, Option<&mut Retaliation>)>) {
    for (mut attackable, mut hp, mut state, mut retaliation) in &mut query {
        let damages = &mut attackable.0;

        'damages: for damage in damages.iter_mut() {
            if hp.is_dead() {
                break 'damages;
            }

            hp.suffer(*damage);

            if hp.is_dead() {
                state.set_died();
                break 'damages;
            } else {
                state.set_stunned();
            }

            if let Some(retaliation) = &mut retaliation {
                retaliation.assign(damage.source());
            }
        }

        damages.clear();
    }
}