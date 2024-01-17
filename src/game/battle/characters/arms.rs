use bevy::prelude::*;
use crate::game::registry::CHARACTER_RADIUS;

pub(super) fn build_arms(_app: &mut App) {}

#[derive(Component, Copy, Clone, Debug)]
pub enum Arms {
    Hand,
    Regular,
    Pole,
}

impl Arms {
    pub fn attack_distance(&self) -> f32 {
        match self {
            Arms::Hand => CHARACTER_RADIUS * 2. * 1.1,
            Arms::Regular => CHARACTER_RADIUS * 2. * 1.5,
            Arms::Pole => CHARACTER_RADIUS * 2. * 2.
        }
    }
}