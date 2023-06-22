use bevy::app::App;
use bevy::prelude::Plugin;
use crate::ecs::resources::build_resources;
use crate::ecs::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        build_resources(app);
        build_systems(app);
    }
}