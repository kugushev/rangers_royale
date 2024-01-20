use std::collections::{HashSet, VecDeque};
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
    for (mut state, mut transform, obstacle, entity) in &mut query {
        if !state.is_active() {
            continue;
        }

        let current = transform.translation.to_vec2();
        let ctx = {
            let mut target = match state.get_activity() {
                Activity::Moving(t) => *t,
                _ => continue,
            };

            let subject_radius = *obstacle.radius();
            target = move_target_closer_if_inside_obstacle(target, &obstacles_q, entity, current, subject_radius);

            PathfindingContext {
                target,
                subject_radius,
                subject_entity: entity,
                visited: HashSet::new(),
            }
        };

        let new_translation = find_best_step(&obstacles_q, &time, current, ctx);

        if let Some((new_translation, finished)) = new_translation {
            if finished {
                state.set_idle();
            }
            transform.translation = vec2_to_vec3(new_translation, transform.translation.z);
        }
    }
}

struct PathfindingContext {
    target: Vec2,
    subject_radius: f32,
    subject_entity: Entity,
    visited: HashSet<(i32, i32)>,
}

impl PathfindingContext {
    pub fn visit_or_false(&mut self, point: Vec2) -> bool {
        let x = point.x * 100.;
        let y = point.y * 100.;
        let key = (x as i32, y as i32);

        self.visited.insert(key)
    }
}

enum PathfindingResult {
    Fail,
    Finish,
    Options(Vec<Vec2>),
}

fn find_best_step(obstacles_q: &Query<(&Obstacle, &GlobalTransform, Entity)>, time: &Res<Time>, current: Vec2, mut ctx: PathfindingContext) -> Option<(Vec2, bool)> {
    let (count, step) = find_best_step_recursive(0, obstacles_q, time, current, &mut ctx)?;

    Some(if count == 1 {
        (ctx.target, true)
    } else {
        (step, false)
    })
}

// todo: add A* greedy optimization
fn find_best_step_recursive(mut steps_count: u8, obstacles_q: &Query<(&Obstacle, &GlobalTransform, Entity)>, time: &Res<Time>, current: Vec2, ctx: &mut PathfindingContext) -> Option<(u8, Vec2)> {
    const MAX_STEPS: u8 = 128;
    if steps_count >= MAX_STEPS {
        return None;
    }
    steps_count += 1;

    let next_result = find_next_translation(obstacles_q, time, current, ctx);
    let next_steps = match next_result {
        PathfindingResult::Fail => return None,
        PathfindingResult::Finish => return Some((steps_count, ctx.target)),
        PathfindingResult::Options(next_steps) => next_steps
    };

    let mut min_step: Option<(u8, Vec2)> = None;
    for next_step in next_steps {
        let next_step_count = find_best_step_recursive(steps_count, obstacles_q, time, next_step, ctx);
        let next_step_count = match next_step_count {
            None => continue,
            Some((count, _)) => count
        };

        let found_better = match min_step {
            None => true,
            Some((min_count, _)) => next_step_count < min_count,
        };
        if found_better {
            min_step = Some((next_step_count, next_step))
        }
    }

    min_step
}

fn find_next_translation(obstacles_q: &Query<(&Obstacle, &GlobalTransform, Entity)>, time: &Res<Time>, current: Vec2, ctx: &mut PathfindingContext) -> PathfindingResult {
    let move_length = time.delta_seconds() * DEFAULT_SPEED;
    let delta = ctx.target - current;

    if delta.length() <= move_length {
        PathfindingResult::Finish
    } else {
        let mut options = vec![];

        let direction = delta.normalize() * move_length;
        let mut new_position = current + direction;
        let mut evade_buffer = VecDeque::new();
        for _ in 0..16 {
            let no_collisions = check_if_not_collisions(ctx, new_position, current, move_length, &obstacles_q, &mut evade_buffer);

            if no_collisions {
                options.push(new_position);
            }

            if let Some(p) = evade_buffer.pop_front() {
                new_position = p;
            } else {
                break;
            }
        }

        if options.is_empty() {
            PathfindingResult::Fail
        } else { PathfindingResult::Options(options) }
    }
}

fn move_target_closer_if_inside_obstacle(target: Vec2, obstacles_q: &Query<(&Obstacle, &GlobalTransform, Entity)>, subject_entity: Entity, current: Vec2, subject_radius: f32) -> Vec2 {
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

fn check_if_not_collisions(ctx: &mut PathfindingContext, new_position: Vec2, current: Vec2, move_length: f32,
                           obstacles_q: &Query<(&Obstacle, &GlobalTransform, Entity)>,
                           evade_buffer: &mut VecDeque<Vec2>) -> bool
{
    for (obstacle, transform, obstacle_entity) in obstacles_q {
        if obstacle_entity == ctx.subject_entity {
            continue;
        }

        let obstacle_radius = *obstacle.radius();
        let obstacle_translation = transform.translation().to_vec2();
        if obstacle_translation.distance(new_position) > ctx.subject_radius + obstacle_radius {
            continue;
        }

        let intersections = find_circle_to_circle_intersections(current, move_length,
                                                                obstacle_translation, current.distance(obstacle_translation));
        match intersections {
            [None, None] => { continue; }
            [Some(intersection), None] => {
                if try_record_intersection(ctx, evade_buffer, intersection) {
                    return false;
                }
                continue;
            }
            [Some(intersection1), Some(intersection2)] => {
                let mut recorded = false;
                if intersection1.distance(ctx.target) < intersection2.distance(ctx.target) {
                    recorded |= try_record_intersection(ctx, evade_buffer, intersection1);
                    recorded |= try_record_intersection(ctx, evade_buffer, intersection2);
                } else {
                    recorded |= try_record_intersection(ctx, evade_buffer, intersection2);
                    recorded |= try_record_intersection(ctx, evade_buffer, intersection1);
                }
                if recorded {
                    return false;
                }
            }
            _ => { error!("Unexpected intersections") }
        }
    }
    return true;
}

fn try_record_intersection(ctx: &mut PathfindingContext, evade_buffer: &mut VecDeque<Vec2>, intersection: Vec2) -> bool {
    if ctx.visit_or_false(intersection) {
        evade_buffer.push_back(intersection);
        return true;
    }
    false
}

fn vec2_to_vec3(vec2: Vec2, z: f32) -> Vec3 {
    Vec3::new(vec2.x, vec2.y, z)
}