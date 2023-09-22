use bevy::prelude::{App, Resource};
use crate::ecs::scenes::GameScene;

pub(super) fn build_world(app: &mut App, scene: GameScene){
    app.insert_resource(WorldMap::default());
}

#[derive(Resource)]
pub(crate) struct WorldMap {
    pub vertical_tiles: u8,
    pub horizontal_tiles: u8,
    pub tile_width: f32,
    pub tile_height: f32,
}

impl WorldMap {
    pub fn get_left(&self) -> f32 { -1.0 * self.vertical_tiles as f32 * self.tile_width / 2.0 }
    pub fn get_right(&self) -> f32 { self.vertical_tiles as f32 * self.tile_width / 2.0 }
    pub fn get_top(&self) -> f32 { self.horizontal_tiles as f32 * self.tile_height / 2.0 }
    pub fn get_bottom(&self) -> f32 { -1.0 * self.horizontal_tiles as f32 * self.tile_height / 2.0 }
}

impl Default for WorldMap {
    fn default() -> Self {
        Self {
            horizontal_tiles: 4,
            vertical_tiles: 8,
            tile_width: 128.0,
            tile_height: 128.0
        }
    }
}
