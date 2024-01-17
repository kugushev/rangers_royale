use bevy::prelude::*;
use crate::game::battle::characters::Character;
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::battle::characters::damage::Damage;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_hit_points(app: &mut App) {

    app.add_systems(First, setup_hp)
        .add_systems(PostUpdate, handle_hp_change);
}

#[derive(Component, Default)]
pub struct HitPoints {
    current: f32,
    changed: bool,
    max: f32,
}

impl HitPoints {
    pub fn suffer(&mut self, damage: &Damage) {
        self.current -= damage.get_amount();
        self.changed = true;
        println!("Suffer: {}/{}", self.current, self.max);
    }

    pub fn is_dead(&self) -> bool {
        if self.max == f32::default() {
            return false;
        }

        self.current <= 0.0
    }
}

fn setup_hp(mut query: Query<(&mut HitPoints, &Character)>) {
    for (mut hit_points, character) in &mut query {
        let max = match character.origin() {
            CharacterOrigin::Red => 20.0,
            CharacterOrigin::Candy => 20.0,
            CharacterOrigin::Knife => 20.0,
            CharacterOrigin::Rose => 20.0,
            CharacterOrigin::Orc => 5.0
        };

        if hit_points.max != max {
            hit_points.max = max;
            hit_points.current = max;
        }
    }
}

fn handle_hp_change(mut query: Query<(&mut HitPoints, &mut CharacterState)>){
    for (mut hp, mut state) in &mut query {
        if !hp.changed {
            continue;
        }

        if hp.is_dead() {
            state.set_died();
        } else {
            // todo: strange place, need to refactor. Let's create extra Component: IncomingDamage with parameters
            state.set_stunned();
        }
        hp.changed = true;
    }
}