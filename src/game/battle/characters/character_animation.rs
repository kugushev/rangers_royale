use bevy::prelude::*;
use derive_getters::Getters;
use crate::game::battle::characters::Character;
use crate::game::battle::characters::character_animations_paths::{CHARACTER_ANIMATIONS_DURATION, CHARACTER_ANIMATIONS_FPS, CharacterAnimationsPaths};
use crate::game::battle::characters::position_tracker::{CharacterDirection, PositionTracker};
use crate::game::common::animation::{AnimationBundle, AnimationIndices, AnimationTimer};
use crate::game::common::layer2d::Layer2d;

pub(super) fn build_character_animation(app: &mut App) {
    app
        .add_systems(Update, toggle_texture_atlas_for_movement)
        .add_systems(Update, toggle_texture_atlas_for_one_shot);
}


#[derive(Bundle)]
pub(super) struct CharacterAnimationBundle {
    animation_bundle: AnimationBundle,
    handles: CharacterAnimationHandles,
    controller: AnimationController,
}

impl CharacterAnimationBundle {
    pub fn new(position: Vec2, paths: &CharacterAnimationsPaths,
               asset_server: &AssetServer, texture_atlases: &mut Assets<TextureAtlas>) -> Self {
        let handles = CharacterAnimationHandles::new(asset_server, texture_atlases, paths);
        Self {
            animation_bundle: AnimationBundle::new(position, Layer2d::Character, CHARACTER_ANIMATIONS_DURATION, CHARACTER_ANIMATIONS_FPS, handles.idle_down.clone_weak()),
            handles,
            controller: AnimationController {
                direction: CharacterDirection::Down,
                speed: MoveSpeed::Idle,
                one_shot: None,
            },
        }
    }
}

#[derive(Component)]
pub struct AnimationController {
    direction: CharacterDirection,
    speed: MoveSpeed,
    one_shot: Option<OneShot>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum MoveSpeed { Idle, _Walk, Run }

struct OneShot {
    animation: OneShotAnimation,
    elapsed: Timer,
    launched: bool,
    suspend: bool
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OneShotAnimation { Swing, Stun }

impl OneShot {
    pub fn get_frames(&self) -> usize {
        match self.animation {
            OneShotAnimation::Swing => 60,
            OneShotAnimation::Stun => 30
        }
    }
}

impl AnimationController {
    pub fn request_one_shot(&mut self, animation: OneShotAnimation, seconds: f32) {
        self.one_shot = Some(OneShot {
            animation,
            elapsed: Timer::from_seconds(seconds, TimerMode::Once),
            launched: false,
            suspend: false
        });
    }

    pub fn suspend_attack(&mut self) {
        if let Some(ref mut x @ OneShot { animation: OneShotAnimation::Swing, .. }) = self.one_shot {
            x.suspend = true;
        }
    }
}


#[derive(Component, Getters)]
struct CharacterAnimationHandles {
    idle_up: Handle<TextureAtlas>,
    idle_down: Handle<TextureAtlas>,
    idle_side: Handle<TextureAtlas>,
    walk_up: Handle<TextureAtlas>,
    walk_down: Handle<TextureAtlas>,
    walk_side: Handle<TextureAtlas>,
    run_up: Handle<TextureAtlas>,
    run_down: Handle<TextureAtlas>,
    run_side: Handle<TextureAtlas>,
    hurt_up: Handle<TextureAtlas>,
    hurt_down: Handle<TextureAtlas>,
    hurt_side: Handle<TextureAtlas>,
    die_up: Handle<TextureAtlas>,
    die_down: Handle<TextureAtlas>,
    die_side: Handle<TextureAtlas>,
    spell_up: Handle<TextureAtlas>,
    spell_down: Handle<TextureAtlas>,
    spell_side: Handle<TextureAtlas>,
    stab_up: Handle<TextureAtlas>,
    stab_down: Handle<TextureAtlas>,
    stab_side: Handle<TextureAtlas>,
    swing_up: Handle<TextureAtlas>,
    swing_down: Handle<TextureAtlas>,
    swing_side: Handle<TextureAtlas>,
}

impl CharacterAnimationHandles {
    fn new(asset_server: &AssetServer, texture_atlases: &mut Assets<TextureAtlas>,
           paths: &CharacterAnimationsPaths) -> Self {
        Self {
            idle_up: load_spritesheet(asset_server, texture_atlases, &paths.idle_up),
            idle_down: load_spritesheet(asset_server, texture_atlases, &paths.idle_down),
            idle_side: load_spritesheet(asset_server, texture_atlases, &paths.idle_side),
            walk_up: load_spritesheet(asset_server, texture_atlases, &paths.walk_up),
            walk_down: load_spritesheet(asset_server, texture_atlases, &paths.walk_down),
            walk_side: load_spritesheet(asset_server, texture_atlases, &paths.walk_side),
            run_up: load_spritesheet(asset_server, texture_atlases, &paths.run_up),
            run_down: load_spritesheet(asset_server, texture_atlases, &paths.run_down),
            run_side: load_spritesheet(asset_server, texture_atlases, &paths.run_side),
            hurt_up: load_spritesheet(asset_server, texture_atlases, &paths.hurt_up),
            hurt_down: load_spritesheet(asset_server, texture_atlases, &paths.hurt_down),
            hurt_side: load_spritesheet(asset_server, texture_atlases, &paths.hurt_side),
            die_up: load_spritesheet(asset_server, texture_atlases, &paths.die_up),
            die_down: load_spritesheet(asset_server, texture_atlases, &paths.die_down),
            die_side: load_spritesheet(asset_server, texture_atlases, &paths.die_side),
            spell_up: load_spritesheet(asset_server, texture_atlases, &paths.spell_up),
            spell_down: load_spritesheet(asset_server, texture_atlases, &paths.spell_down),
            spell_side: load_spritesheet(asset_server, texture_atlases, &paths.spell_side),
            stab_up: load_spritesheet(asset_server, texture_atlases, &paths.stab_up),
            stab_down: load_spritesheet(asset_server, texture_atlases, &paths.stab_down),
            stab_side: load_spritesheet(asset_server, texture_atlases, &paths.stab_side),
            swing_up: load_spritesheet(asset_server, texture_atlases, &paths.swing_up),
            swing_down: load_spritesheet(asset_server, texture_atlases, &paths.swing_down),
            swing_side: load_spritesheet(asset_server, texture_atlases, &paths.swing_side),
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

fn toggle_texture_atlas_for_movement(mut query: Query<(&mut Handle<TextureAtlas>, &mut TextureAtlasSprite, &PositionTracker, &CharacterAnimationHandles, &mut AnimationController), With<Character>>) {
    for (mut texture_atlas, mut sprite, position_tracker, handles, mut controller) in &mut query {
        if controller.one_shot.is_some() {
            continue;
        }

        let direction = *position_tracker.direction();

        let speed = match *position_tracker.speed() {
            s if s <= f32::EPSILON => MoveSpeed::Idle,
            s if s > f32::EPSILON => MoveSpeed::Run,
            s => {
                error!("Unexpected speed {s}");
                MoveSpeed::Idle
            }
        };

        let changed = controller.direction != direction || controller.speed != speed;

        if !changed {
            continue;
        }

        // todo: add anti-shake - if changes makes every frame (Left-Up-Left), don't change atlas/flip

        *texture_atlas = match (direction, speed) {
            (CharacterDirection::Up, MoveSpeed::Run) => handles.run_up.clone_weak(),
            (CharacterDirection::Up, MoveSpeed::_Walk) => handles.walk_up.clone_weak(),
            (CharacterDirection::Up, MoveSpeed::Idle) => handles.idle_up.clone_weak(),
            (CharacterDirection::Down, MoveSpeed::Run) => handles.run_down.clone_weak(),
            (CharacterDirection::Down, MoveSpeed::_Walk) => handles.walk_down.clone_weak(),
            (CharacterDirection::Down, MoveSpeed::Idle) => handles.idle_down.clone_weak(),
            (CharacterDirection::Left, MoveSpeed::Run) => handles.run_side.clone_weak(),
            (CharacterDirection::Left, MoveSpeed::_Walk) => handles.walk_side.clone_weak(),
            (CharacterDirection::Left, MoveSpeed::Idle) => handles.idle_side.clone_weak(),
            (CharacterDirection::Right, MoveSpeed::Run) => handles.run_side.clone_weak(),
            (CharacterDirection::Right, MoveSpeed::_Walk) => handles.walk_side.clone_weak(),
            (CharacterDirection::Right, MoveSpeed::Idle) => handles.idle_side.clone_weak(),
        };
        sprite.flip_x = direction == CharacterDirection::Left;

        controller.direction = direction;
        controller.speed = speed;
    }
}

fn toggle_texture_atlas_for_one_shot(
    mut query: Query<(
        &mut Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &PositionTracker,
        &CharacterAnimationHandles,
        &mut AnimationController,
        &mut AnimationIndices,
        &mut AnimationTimer), With<Character>>, time: Res<Time>) {
    for (mut texture_atlas, mut sprite, position_tracker, handles, mut controller, mut indices, mut timer) in &mut query {
        let one_shot = match &mut controller.one_shot {
            Some(x) => x,
            None => { continue; }
        };

        let mut configure_animation = |duration: f32, frames| {
            let fps = duration / frames as f32;
            indices.setup(frames);
            timer.reset(fps);
        };

        let direction = *position_tracker.direction();

        if !one_shot.launched {
            *texture_atlas = match (direction, one_shot.animation) {
                (CharacterDirection::Up, OneShotAnimation::Swing) => handles.swing_up.clone_weak(),
                (CharacterDirection::Down, OneShotAnimation::Swing) => handles.swing_down.clone_weak(),
                (CharacterDirection::Left, OneShotAnimation::Swing) => handles.swing_side.clone_weak(),
                (CharacterDirection::Right, OneShotAnimation::Swing) => handles.swing_side.clone_weak(),
                (CharacterDirection::Up, OneShotAnimation::Stun) => handles.hurt_up.clone_weak(),
                (CharacterDirection::Down, OneShotAnimation::Stun) => handles.hurt_down.clone_weak(),
                (CharacterDirection::Left, OneShotAnimation::Stun) => handles.hurt_side.clone_weak(),
                (CharacterDirection::Right, OneShotAnimation::Stun) => handles.hurt_side.clone_weak(),
            };
            sprite.flip_x = direction == CharacterDirection::Left;
            sprite.index = 0;

            configure_animation(one_shot.elapsed.duration().as_secs_f32(), one_shot.get_frames());

            one_shot.launched = true;
            continue;
        }

        one_shot.elapsed.tick(time.delta());

        if one_shot.elapsed.finished() || one_shot.suspend {
            *texture_atlas = match direction {
                CharacterDirection::Up => handles.idle_up.clone_weak(),
                CharacterDirection::Down => handles.idle_down.clone_weak(),
                CharacterDirection::Left => handles.idle_side.clone_weak(),
                CharacterDirection::Right => handles.idle_side.clone_weak(),
            };
            sprite.flip_x = direction == CharacterDirection::Left;
            sprite.index = 0;

            configure_animation(CHARACTER_ANIMATIONS_DURATION, CHARACTER_ANIMATIONS_FPS);

            controller.one_shot = None;
        }
    }
}
