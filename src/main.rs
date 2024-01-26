mod game;

use std::time::Duration;
use bevy::asset::ChangeWatcher;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            // .set(AssetPlugin {
            //     watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(1)),
            //     ..default()
            // })
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: [1920., 1080.].into(),
                    title: "Rangers Royale".to_string(),
                    ..default()
                }),
                ..default()
            }))
        .add_plugins(TilemapPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}


