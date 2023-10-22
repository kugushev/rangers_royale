use std::collections::VecDeque;
use bevy::prelude::*;
use crate::game::battle::characters::character_state::activity::Activity;
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::common::obstacle::Obstacle;
use crate::game::utils::{find_circle_to_circle_intersections, Vec3Ex};

pub(super) fn build_moving(app: &mut App)
{
    app.add_systems(Update, handle_move);
}

const DEFAULT_SPEED: f32 = 100.0;


fn handle_move(mut query: Query<(&mut CharacterState, &mut Transform, &Obstacle, Entity)>, obstacles_q: Query<(&Obstacle, &GlobalTransform, Entity)>, time: Res<Time>) {

    // todo: refactor
    for (mut state, mut transform, obstacle, entity) in &mut query {
        if !state.is_active() {
            continue;
        }

        let mut target = match state.get_activity() {
            Activity::Moving(t) => *t,
            _ => { continue; }
        };

        let current = transform.translation.to_vec2();
        let z = transform.translation.z;

        let subject_radius = *obstacle.radius();

        target = move_target_closer_if_not_reachable(target, &obstacles_q, entity, current, subject_radius);

        let move_length = time.delta_seconds() * DEFAULT_SPEED;
        let delta = target - current;
        if delta.length() <= move_length {
            transform.translation = vec2_to_vec3(target, z);
            state.set_idle();
        } else {
            let direction = delta.normalize() * move_length;
            let mut new_position = current + direction;

            let mut final_new_position = Some(new_position);
            let mut evade_buffer = VecDeque::new();
            'collisions: for _ in 0..16 {
                let no_collisions = check_if_not_collisions(new_position, current, target, move_length,
                                                            subject_radius,
                                                            entity,
                                                            &obstacles_q,
                                                            &mut evade_buffer);

                if no_collisions {
                    final_new_position = Some(new_position);
                    break 'collisions;
                }

                if let Some(p) = evade_buffer.pop_front() {
                    new_position = p;
                }
            }

            match final_new_position {
                None => { println!("Shit!") }
                Some(p) => transform.translation = vec2_to_vec3(p, z)
            }
        }
    }

    fn vec2_to_vec3(vec2: Vec2, z: f32) -> Vec3 {
        Vec3::new(vec2.x, vec2.y, z)
    }
}

fn move_target_closer_if_not_reachable(target: Vec2, obstacles_q: &Query<(&Obstacle, &GlobalTransform, Entity)>, subject_entity: Entity, current: Vec2, subject_radius: f32) -> Vec2 {
    for (obstacle, transform, obstacle_entity) in obstacles_q {
        if obstacle_entity == subject_entity {
            continue;
        }

        let obstacle_position = transform.translation().to_vec2();
        let intersection_radius = *obstacle.radius() + subject_radius;
        if obstacle_position.distance(target) > intersection_radius {
            continue;
        }
        let delta = obstacle_position - current;
        let new_delta_length = (delta.length() - intersection_radius).max(0.);
        let clamped_direction = delta.normalize() * new_delta_length;
        return current + clamped_direction;
    }

    target
}

fn check_if_not_collisions(new_position: Vec2, current: Vec2, target: Vec2, move_length: f32,
                           subject_radius: f32, subject_entity: Entity,
                           obstacles_q: &Query<(&Obstacle, &GlobalTransform, Entity)>,
                           evade_buffer: &mut VecDeque<Vec2>) -> bool
{
    for (obstacle, transform, obstacle_entity) in obstacles_q {
        if obstacle_entity == subject_entity {
            continue;
        }

        let obstacle_radius = *obstacle.radius();
        let obstacle_translation = transform.translation().to_vec2();
        if obstacle_translation.distance(new_position) > subject_radius + obstacle_radius {
            continue;
        }

        let intersections = find_circle_to_circle_intersections(current, move_length,
                                                                obstacle_translation, current.distance(obstacle_translation));
        match intersections {
            [None, None] => {
                continue;
            }
            [Some(intersection), None] => {
                evade_buffer.push_back(intersection);
                return false;
            }
            [Some(intersection1), Some(intersection2)] => {
                if intersection1.distance(target) < intersection2.distance(target) {
                    evade_buffer.push_back(intersection1);
                    evade_buffer.push_back(intersection2);
                } else {
                    evade_buffer.push_back(intersection2);
                    evade_buffer.push_back(intersection1);
                }
                return false;
            }
            _ => { error!("Unexpected intersections") }
        }
    }
    return true;
}