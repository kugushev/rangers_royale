
mod game;

use bevy::DefaultPlugins;
use bevy::prelude::{App, ImagePlugin, PluginGroup};
use bevy_ecs_tilemap::TilemapPlugin;
use crate::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(GamePlugin)
        .run();
}


