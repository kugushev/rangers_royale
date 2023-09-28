
mod game;

use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use crate::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: [1280., 800.].into(),
                    title: "Rangers Royale".to_string(),
                    ..default()
                }),
                ..default()
            }))
        .add_plugins(TilemapPlugin)
        .add_plugins(GamePlugin)
        .run();
}


