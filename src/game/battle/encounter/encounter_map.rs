use bevy::prelude::*;
use bevy::asset::{AssetServer, Handle};
use bevy_ecs_tilemap::map::{TilemapId, TilemapSize, TilemapTexture, TilemapTileSize, TilemapType};
use bevy_ecs_tilemap::prelude::{get_tilemap_center_transform, TileBundle, TilePos, TileStorage, TileTextureIndex};
use bevy_ecs_tilemap::TilemapBundle;
use crate::game::game_mode::GameMode;

pub(super) fn build_encounter_map(app: &mut App) {
    app.insert_resource(EncounterMap::default())
        .add_systems(OnEnter(GameMode::Battle), draw_tiles)
        .add_systems(OnExit(GameMode::Battle), clear_tiles);
}

#[derive(Resource)]
pub(crate) struct EncounterMap {
    pub vertical_tiles: u32,
    pub horizontal_tiles: u32,
    pub tile_width: f32,
    pub tile_height: f32,
    pub index: u32,
    handle: Option<Handle<Image>>,
}

impl EncounterMap {
    pub fn get_height(&self) -> f32 { self.vertical_tiles as f32 * self.tile_height }
    pub fn get_width(&self) -> f32 { self.horizontal_tiles as f32 * self.tile_width }
    pub fn get_left(&self) -> f32 { -1.0 * self.get_height() / 2.0 }
    pub fn get_right(&self) -> f32 { self.get_height() / 2.0 }
    pub fn get_top(&self) -> f32 { self.get_width() / 2.0 }
    pub fn get_bottom(&self) -> f32 { -1.0 * self.get_width() / 2.0 }
}

impl Default for EncounterMap {
    fn default() -> Self {
        Self {
            horizontal_tiles: 16,
            vertical_tiles: 10,
            tile_width: 128.0,
            tile_height: 128.0,
            index: 0,
            handle: None,
        }
    }
}

fn draw_tiles(mut encounter_map: ResMut<EncounterMap>, mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("draw tiles");

    let texture_handle = encounter_map.handle.get_or_insert_with(|| asset_server.load("paid/tiles/Dungeon@128x128.png"));
    let texture_handle = texture_handle.clone_weak();

    let map_size = TilemapSize { x: encounter_map.horizontal_tiles, y: encounter_map.vertical_tiles };
    let mut tile_storage = TileStorage::empty(map_size);
    let root_entity_id = commands.spawn_empty()
        .with_children(|builder| {
            let tilemap_entity_id = builder.parent_entity();
            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let tile_pos = TilePos { x, y };
                    let tile_entity = builder
                        .spawn(TileBundle {
                            position: tile_pos,
                            tilemap_id: TilemapId(tilemap_entity_id),
                            texture_index: TileTextureIndex(encounter_map.index),
                            ..Default::default()
                        })
                        .id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            }
        }).id();

    let tile_size = TilemapTileSize { x: 128.0, y: 128.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(root_entity_id).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    encounter_map.index += 1;
}

fn clear_tiles(mut query: Query<Entity, With<TileStorage>>, mut commands: Commands) {
    println!("clear tiles");
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
