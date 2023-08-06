mod game_plugin;
mod ecs;
mod registry;

use bevy::DefaultPlugins;
use bevy::prelude::{App, ImagePlugin, PluginGroup};
use bevy_ecs_tilemap::TilemapPlugin;
use crate::game_plugin::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_plugin(GamePlugin)
        .run();
}


