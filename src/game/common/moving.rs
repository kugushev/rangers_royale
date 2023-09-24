use bevy::prelude::*;
use crate::game::utils::Vec3Ex;

pub(super) fn build_moving(app: &mut App) {
    app.add_systems(Update, handle_move);
}

const DEFAULT_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct MoveCommand {
    pub target: Option<Vec2>,
    pub speed: f32,
}

impl Default for MoveCommand {
    fn default() -> Self {
        Self {
            target: None,
            speed: DEFAULT_SPEED,
        }
    }
}

fn handle_move(mut query: Query<(&mut MoveCommand, &mut Transform)>, time: Res<Time>) {
    for (mut command, mut transform) in &mut query {
        let target = match command.target {
            None => { continue; }
            Some(t) => t
        };

        let current = transform.translation.to_vec2();
        let z = transform.translation.z;

        let move_length = time.delta_seconds() * command.speed;
        let delta = target - current;
        if delta.length() <= move_length {
            transform.translation = vec2_to_vec3(target, z);
            command.target = None;
        } else {
            let direction = delta.normalize() * move_length;
            let new_position = current + direction;

            transform.translation = vec2_to_vec3(new_position, z);
        }
    }

    fn vec2_to_vec3(vec2: Vec2, z: f32) -> Vec3 {
        Vec3::new(vec2.x, vec2.y, z)
    }
}