use bevy::prelude::*;
use crate::game::battle::build_battle;
use crate::game::common::build_common;
use crate::game::scenes::build_scenes;

mod battle;
mod common;
mod scenes;
pub mod utils;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        build_scenes(app);
        build_common(app);
        build_battle(app);
    }
}




