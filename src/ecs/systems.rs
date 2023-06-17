use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::ecs::components::{Character, Position};

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle = asset_server.load("paid/Grassland@128x128.png"); // todo: load and put to Res to use handle in a shared way

    let map_size = TilemapSize { x: 8, y: 8 };
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

    let tile_size = TilemapTileSize { x: 128.0, y: 128.0 };
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

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Position { x: 0.0, y: 0.0 };
    let sprite = SpriteBundle {
        texture: asset_server.load("paid/player.png"),
        transform: Transform::from_xyz(position.x, position.y, 2.0),
        ..default()
    };

    commands.spawn(Character {
        position,
        sprite,
    });
}