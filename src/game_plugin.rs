use bevy::app::App;
use bevy::prelude::Plugin;
use crate::ecs::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        build_systems(app);
    }
}