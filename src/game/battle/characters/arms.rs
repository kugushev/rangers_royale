use bevy::prelude::*;
use derive_getters::Getters;

pub(super) fn build_arms(_app: &mut App) {

}

#[derive(Component, Getters)]
pub struct Arms {
    range: f32
}

impl Arms {
    pub fn new(range: f32) -> Self { Self { range } }
}