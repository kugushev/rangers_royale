mod game_plugin;
mod ecs;

use bevy::DefaultPlugins;
use bevy::prelude::{App, Component, ImagePlugin, PluginGroup};
use bevy_ecs_tilemap::TilemapPlugin;
use crate::game_plugin::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_plugin(GamePlugin)
        .run();
}


