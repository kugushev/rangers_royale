use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::common::gizmos_manager::GizmosManager;
use crate::game::players::host_cursor::HostCursor;
use crate::game::utils::Vec3Ex;

pub(super) fn build_cursor_collider(app: &mut App) {
    app.add_systems(PreUpdate, hover_test);

    if cfg!(debug_assertions) {
        app.add_systems(PostUpdate, draw_gizmos);
    }
}

#[derive(Component, Getters)]
pub struct CursorCollider {
    hovered: bool,
    size: Vec2,
    shift: Vec2,
}

impl CursorCollider {
    pub fn new() -> Self {
        Self {
            hovered: false,
            size: Vec2::new(60., 100.),
            shift: Vec2::new(0., 40.),
        }
    }
}

fn hover_test(mut query: Query<(&mut CursorCollider, &GlobalTransform)>, cursor: Res<HostCursor>) {
    for (mut collider, transform) in &mut query {
        let translation = transform.translation();
        let left = translation.x + collider.shift.x - (collider.size.x / 2.);
        let right = translation.x + collider.shift.x + (collider.size.x / 2.);
        let top = translation.y + collider.shift.y - (collider.size.y / 2.);
        let bottom = translation.y + collider.shift.y + (collider.size.y / 2.);

        let Vec2 { x, y } = *cursor.position();
        collider.hovered = x >= left && x <= right && y >= top && y <= bottom;
    }
}

fn draw_gizmos(query: Query<(&CursorCollider, &GlobalTransform)>, mut gizmos: Gizmos, gizmos_manager: Res<GizmosManager>) {
    if !gizmos_manager.show() {
        return;
    }

    for (collider, global_transform) in &query {
        let mut position = global_transform.translation().to_vec2();
        position.x += collider.shift.x;
        position.y += collider.shift.y;
        gizmos.rect_2d(position, 0., collider.size, Color::AQUAMARINE);
    }
}

