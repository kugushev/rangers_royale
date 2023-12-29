use bevy::prelude::*;
use crate::game::battle::characters::Character;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_damage(app: &mut App) {
    app.add_systems(First, setup_damage);
}

#[derive(Component, Default)]
pub struct Damage {
    amount: f32,
}

impl Damage {
    pub fn get_amount(&self) -> f32 {
        self.amount
    }
}

fn setup_damage(mut query: Query<(&mut Damage, &Character)>) {
    for (mut damage, character) in &mut query {
        damage.amount = match character.origin() {
            CharacterOrigin::Red => 2.0,
            CharacterOrigin::Candy => 1.5,
            CharacterOrigin::Knife => 1.0,
            CharacterOrigin::Rose => 0.5,
        }
    }
}