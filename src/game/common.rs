use bevy::prelude::*;
use crate::game::common::animation::build_animation;
use crate::game::common::camera::build_camera;
use crate::game::common::ellipse_collider::build_ellipse_collider;
use crate::game::common::layer2d::build_layer2d;
use crate::game::common::moving::build_moving;
use crate::game::common::selection_mark::build_selection_mark;

pub mod animation;
pub mod moving;
pub mod ellipse_collider;
pub mod camera;
pub mod layer2d;
pub mod selection_mark;

pub(super) fn build_common(app: &mut App) {
    build_animation(app);
    build_moving(app);
    build_ellipse_collider(app);
    build_camera(app);
    build_layer2d(app);
    build_selection_mark(app);
}
