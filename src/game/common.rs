use bevy::prelude::*;
use crate::game::common::animation::build_animation;
use crate::game::common::camera::build_camera;
use crate::game::common::ellipse_collider::build_ellipse_collider;
use crate::game::common::game_cursor::build_cursor;
use crate::game::common::layer2d::build_layer2d;
use crate::game::common::player_input::build_player_input;
use crate::game::common::moving::build_moving;

pub mod animation;
pub mod moving;
pub mod player_input;
pub mod ellipse_collider;
pub mod game_cursor;
pub mod camera;
pub mod layer2d;

pub(super) fn build_common(app: &mut App) {
    build_animation(app);
    build_moving(app);
    build_player_input(app);
    build_ellipse_collider(app);
    build_camera(app);
    build_cursor(app);
    build_layer2d(app);
}
