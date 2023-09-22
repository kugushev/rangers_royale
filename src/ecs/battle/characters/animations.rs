use bevy::prelude::*;
use derive_getters::Getters;
use crate::ecs::common::spritesheet_animations::AnimationBundle;
use crate::ecs::scenes::GameScene;
use crate::registry::character_animations_paths::{CHARACTER_ANIMATIONS_FPS, CharacterAnimationsPaths};

pub(super) fn build_player_characters(app: &mut App, scene: GameScene) {
    // todo: add system change_animation_handle and component character_animation_controller
}


#[derive(Bundle)]
pub(super) struct CharacterAnimationBundle {
    pub animation_bundle: AnimationBundle,
    pub handles: CharacterAnimationHandles,
}

impl CharacterAnimationBundle {
    pub fn new(position: Vec2, paths: &CharacterAnimationsPaths,
               asset_server: &AssetServer, texture_atlases: &mut Assets<TextureAtlas>) -> Self {
        let handles = CharacterAnimationHandles::new(asset_server, texture_atlases, paths);
        Self {
            animation_bundle: AnimationBundle::new(position, CHARACTER_ANIMATIONS_FPS, handles.idle_down.clone_weak()),
            handles,
        }
    }
}

#[derive(Component, Getters)]
pub struct CharacterAnimationHandles {
    idle_up: Handle<TextureAtlas>,
    idle_down: Handle<TextureAtlas>,
    idle_side: Handle<TextureAtlas>,
    run_up: Handle<TextureAtlas>,
    run_down: Handle<TextureAtlas>,
    run_side: Handle<TextureAtlas>,
    hurt_up: Handle<TextureAtlas>,
    hurt_down: Handle<TextureAtlas>,
    hurt_side: Handle<TextureAtlas>,
    die_up: Handle<TextureAtlas>,
    die_down: Handle<TextureAtlas>,
    die_side: Handle<TextureAtlas>,
}

impl CharacterAnimationHandles {
    fn new(asset_server: &AssetServer, texture_atlases: &mut Assets<TextureAtlas>,
           paths: &CharacterAnimationsPaths) -> Self {
        Self {
            idle_up: load_spritesheet(asset_server, texture_atlases, &paths.idle_up),
            idle_down: load_spritesheet(asset_server, texture_atlases, &paths.idle_down),
            idle_side: load_spritesheet(asset_server, texture_atlases, &paths.idle_side),
            run_up: load_spritesheet(asset_server, texture_atlases, &paths.run_up),
            run_down: load_spritesheet(asset_server, texture_atlases, &paths.run_down),
            run_side: load_spritesheet(asset_server, texture_atlases, &paths.run_side),
            hurt_up: load_spritesheet(asset_server, texture_atlases, &paths.hurt_up),
            hurt_down: load_spritesheet(asset_server, texture_atlases, &paths.hurt_down),
            hurt_side: load_spritesheet(asset_server, texture_atlases, &paths.hurt_side),
            die_up: load_spritesheet(asset_server, texture_atlases, &paths.die_up),
            die_down: load_spritesheet(asset_server, texture_atlases, &paths.die_down),
            die_side: load_spritesheet(asset_server, texture_atlases, &paths.die_side),
        }
    }
}

fn load_spritesheet(asset_server: &AssetServer, texture_atlases: &mut Assets<TextureAtlas>, path: &String) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(path);
    let texture_atlas = TextureAtlas::from_grid(texture_handle,
                                                Vec2::new(256.0, 256.0),
                                                6,
                                                10,
                                                None,
                                                None);
    texture_atlases.add(texture_atlas)
}