use bevy::app::App;
use bevy::prelude::Plugin;
use crate::ecs::systems::{setup_map, setup, setup_player};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_startup_system(setup_map)
            .add_startup_system(setup_player);
    }
}