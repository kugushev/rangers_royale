mod player;
mod characters;
mod ui;
mod projectiles;
mod spritesheet_animations;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::ecs::battle::world::WorldMap;
use crate::x_old::systems::characters::build_characters_systems;
use crate::x_old::systems::player::build_player_systems;
use crate::x_old::systems::projectiles::build_projectiles_systems;
use crate::x_old::systems::spritesheet_animations::build_spritesheet_animations_systems;
use crate::x_old::systems::ui::build_ui_systems;

pub(crate) fn build_systems(app: &mut App) {
    app.add_startup_system(setup)
        .add_startup_system(setup_map);

    build_ui_systems(app);
    build_characters_systems(app);
    build_player_systems(app);
    build_projectiles_systems(app);
    build_spritesheet_animations_systems(app)
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>, world_map: Res<WorldMap>) {
    let texture_handle = asset_server.load("paid/Grassland@128x128.png"); // todo: load and put to Res to use handle in a shared way

    let map_size = TilemapSize {
        x: world_map.vertical_tiles as u32,
        y: world_map.horizontal_tiles as u32,
    };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(17),
                ..Default::default()
            }).id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: world_map.tile_width, y: world_map.tile_height };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}