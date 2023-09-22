use bevy::prelude::*;
use crate::ecs::common::spritesheet_animations::build_spritesheet_animations_systems;

pub mod spritesheet_animations;

pub(super) fn build_common(app: &mut App) {
    build_spritesheet_animations_systems(app)
}