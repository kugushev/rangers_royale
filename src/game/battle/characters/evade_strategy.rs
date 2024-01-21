use bevy::prelude::*;
use rand::random;

pub(super) fn build_evade_strategy(app: &mut App) {}

#[derive(Component)]
pub enum EvadeStrategy {
    Left,
    Right,
}

impl EvadeStrategy {
    pub fn new() -> Self {
        let random: u8 = random();
        if random % 2 == 0 {
            EvadeStrategy::Left
        } else { EvadeStrategy::Right }
    }
}
