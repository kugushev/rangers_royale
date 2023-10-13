use bevy::prelude::*;
use crate::game::common::animation::build_animation;
use crate::game::common::camera::build_camera;
use crate::game::common::gizmos_manager::build_gizmos_manager;
use crate::game::common::cursor_collider::build_cursor_collider;
use crate::game::common::layer2d::build_layer2d;
use crate::game::common::obstacle::build_obstacle;

pub mod animation;
pub mod cursor_collider;
pub mod camera;
pub mod layer2d;
pub mod gizmos_manager;
pub mod obstacle;

pub(super) fn build_common(app: &mut App) {
    build_animation(app);
    build_cursor_collider(app);
    build_camera(app);
    build_layer2d(app);
    build_gizmos_manager(app);
    build_obstacle(app);
}
