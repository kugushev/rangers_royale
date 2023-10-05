use bevy::prelude::*;
use crate::game::common::animation::build_animation;
use crate::game::common::camera::build_camera;
use crate::game::common::ellipse_collider::build_ellipse_collider;
use crate::game::common::layer2d::build_layer2d;
use crate::game::common::moving::build_moving;

pub mod animation;
pub mod moving;
pub mod ellipse_collider;
pub mod camera;
pub mod layer2d;

pub(super) fn build_common(app: &mut App) {
    build_animation(app);
    build_moving(app);
    build_ellipse_collider(app);
    build_camera(app);
    build_layer2d(app);
}
