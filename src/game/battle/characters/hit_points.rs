use bevy::prelude::*;
use crate::game::battle::characters::Character;
use crate::game::battle::characters::damage::Damage;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_hit_points(app: &mut App) {
    app.add_systems(First, setup_hp);
}

#[derive(Component, Default)]
pub struct HitPoints {
    current: f32,
    max: f32,
}

impl HitPoints {
    pub fn suffer(&mut self, damage: &Damage) {
        self.current -= damage.get_amount();
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
            CharacterOrigin::Red => 5.0,
            CharacterOrigin::Candy => 7.0,
            CharacterOrigin::Knife => 10.0,
            CharacterOrigin::Rose => 15.0,
        };

        if hit_points.max != max {
            hit_points.max = max;
            hit_points.current = max;
        }
    }
}