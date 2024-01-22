use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::battle::characters::Character;
use crate::game::battle::value_objects::Damage;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_attacker(app: &mut App) {
    app.add_systems(First, setup_damage);
}

#[derive(Component, Default, Getters)]
pub struct Attacker {
    damage_amount: f32,
}


fn setup_damage(mut query: Query<(&mut Attacker, &Character)>) {
    for (mut damage, character) in &mut query {

        // damage.damage_amount = 0.;
        damage.damage_amount = match character.origin() {
            CharacterOrigin::Red => 1.0,
            CharacterOrigin::Candy => 1.0,
            CharacterOrigin::Knife => 1.0,
            CharacterOrigin::Rose => 1.0,
            CharacterOrigin::Orc => 1.0
        }
    }
}