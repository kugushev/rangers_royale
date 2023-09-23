use bevy::prelude::*;
use derive_getters::Getters;
use crate::ecs::battle::characters::Character;
use crate::ecs::battle::characters::position_tracking::{CharacterDirection, PositionTracker};
use crate::ecs::common::animation::AnimationBundle;
use crate::ecs::scenes::GameScene;
use crate::registry::character_animations_paths::{CHARACTER_ANIMATIONS_FPS, CharacterAnimationsPaths};

pub(super) fn build_character_animation(app: &mut App, scene: GameScene) {
    app.add_systems(Update, handle_direction.run_if(in_state(scene)));
}


#[derive(Bundle)]
pub(super) struct CharacterAnimationBundle {
    pub animation_bundle: AnimationBundle,
    handles: CharacterAnimationHandles,
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

fn handle_direction(mut query: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &PositionTracker, &CharacterAnimationHandles), With<Character>>) {
    for (mut texture_atlas, mut sprite, position_tracker, handles) in &mut query {
        *texture_atlas = match position_tracker.direction() {
            CharacterDirection::Up => handles.idle_up.clone_weak(),
            CharacterDirection::Down => handles.idle_down.clone_weak(),
            CharacterDirection::Left => handles.idle_side.clone_weak(),
            CharacterDirection::Right => handles.idle_side.clone_weak(),
        };

        sprite.flip_x = *position_tracker.direction() == CharacterDirection::Left;
    }
}

#[derive(Component, Getters)]
struct CharacterAnimationHandles {
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