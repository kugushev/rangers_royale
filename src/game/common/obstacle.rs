use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::common::gizmos_manager::GizmosManager;
use crate::game::utils::Vec3Ex;

pub(super) fn build_obstacle(app: &mut App) {
    if cfg!(debug_assertions) {
        app.add_systems(PostUpdate, draw_gizmos);
    }
}

#[derive(Component, Getters)]
pub struct Obstacle {
    radius: f32,
}

impl Obstacle {
    pub fn new(radius: f32) -> Self { Self { radius } }
}

fn draw_gizmos(query: Query<(&Obstacle, &GlobalTransform)>, mut gizmos: Gizmos, gizmos_manager: Res<GizmosManager>) {
    if !gizmos_manager.show() {
        return;
    }

    for (obstacle, global_transform) in &query {
        let position = global_transform.translation().to_vec2();
        gizmos.circle_2d(position, obstacle.radius, Color::CYAN);
    }
}