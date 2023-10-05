use bevy::prelude::*;
use crate::game::utils::Vec3Ex;

pub(super) fn build_rect_collider(app: &mut App) {
    app.add_systems(PostUpdate, draw_gizmos);
}

#[derive(Component)]
pub struct EllipseCollider {
    size: Vec2,
    shift: Vec2
}

impl EllipseCollider {
    pub fn new() -> Self {
        Self {
            size: Vec2::new(60., 100.),
            shift: Vec2::new(0., 40.),
        }
    }
}

fn draw_gizmos(query: Query<(&EllipseCollider, &GlobalTransform)>, mut gizmos: Gizmos, time: Res<Time>) {
    for (collider, global_transform) in &query {
        let mut position = global_transform.translation().to_vec2();
        position.x += collider.shift.x;
        position.y += collider.shift.y;
        gizmos.rect_2d(position, 0., collider.size, Color::AQUAMARINE);
    }
}