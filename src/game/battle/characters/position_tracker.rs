use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::battle::characters::Character;
use crate::game::utils::Vec3Ex;

pub(super) fn build_position_tracking(app: &mut App) {
    app.add_systems(Update, track_character_position);
}

#[derive(Component, Getters, Default)]
pub struct PositionTracker {
    #[getter(skip)]
    previous: Option<Vec2>,
    direction: CharacterDirection,
    speed: f32,
}

impl PositionTracker {
    pub fn look_at(&mut self, target: Vec2, position: Vec2) {
        if let Some(direction) = get_direction(position, target) {
            self.direction = direction;
            self.previous = None;
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
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
            if let Some(direction) = get_direction(previous, current) {
                position_tracker.direction = direction;
            }
            position_tracker.speed = previous.distance(current).abs();
        }

        position_tracker.previous = Some(transform.translation.to_vec2());
    }
}

fn get_direction(from: Vec2, to: Vec2) -> Option<CharacterDirection> {
    let direction_vec = to - from;
    let look_plus_45 = Vec2::new(1., 1.);
    let look_minus_45 = Vec2::new(1., -1.);

    let dot_plus_45 = direction_vec.dot(look_plus_45);
    let dot_minus_45 = direction_vec.dot(look_minus_45);

    if dot_plus_45.abs() <= f32::EPSILON && dot_minus_45.abs() <= f32::EPSILON {
        None
    } else if dot_plus_45 > 0.0 && dot_minus_45 > 0.0 {
        Some(CharacterDirection::Right)
    } else if dot_plus_45 > 0.0 && dot_minus_45 <= 0.0 {
        Some(CharacterDirection::Up)
    } else if dot_plus_45 <= 0.0 && dot_minus_45 > 0.0 {
        Some(CharacterDirection::Down)
    } else if dot_plus_45 <= 0.0 && dot_minus_45 <= 0.0 {
        Some(CharacterDirection::Left)
    } else {
        Some(CharacterDirection::default())
    }
}

