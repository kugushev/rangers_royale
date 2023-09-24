use bevy::prelude::*;
use derive_getters::Getters;
use crate::ecs::battle::characters::Character;
use crate::utils::Vec3toVec2;

pub(super) fn build_position_tracking(app: &mut App) {
    app.add_systems(Update, track_character_position);
}

#[derive(Component, Getters, Default)]
pub struct PositionTracker {
    #[getter(skip)]
    previous: Option<Vec2>,
    direction: CharacterDirection,
    speed: f32
}

#[derive(Copy, Clone, PartialEq)]
pub enum CharacterDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Default for CharacterDirection {
    fn default() -> Self { Self::Down }
}

fn track_character_position(mut query: Query<(&mut PositionTracker, &Transform), With<Character>>) {
    for (mut position_tracker, transform) in &mut query {
        if let Some(previous) = position_tracker.previous {
            let current = transform.translation.to_vec2();
            let direction_vec = current - previous;
            let look_plus_45 = Vec2::new(1., 1.);
            let look_minus_45 = Vec2::new(1., -1.);

            let dot_plus_45 = direction_vec.dot(look_plus_45);
            let dot_minus_45 = direction_vec.dot(look_minus_45);

            position_tracker.direction = match (dot_plus_45, dot_minus_45) {
                (0.0, 0.0) => position_tracker.direction,
                (0.0.., 0.0..) => CharacterDirection::Right,
                (0.0.., ..=0.0) => CharacterDirection::Up,
                (..=0.0, 0.0..) => CharacterDirection::Down,
                (..=0.0, ..=0.0) => CharacterDirection::Left,
                (_, _) => CharacterDirection::default(),
            };
            position_tracker.speed = previous.distance(current).abs();
        }

        position_tracker.previous = Some(transform.translation.to_vec2());
    }
}

