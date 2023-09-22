use bevy::prelude::*;
use crate::ecs::battle::build_battle;
use crate::ecs::camera::build_camera;
use crate::ecs::common::build_common;
use crate::ecs::scenes::build_scenes;

mod battle;
mod common;
mod scenes;
mod camera;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        build_camera(app);
        build_scenes(app);
        build_common(app);
        build_battle(app);
    }
}




