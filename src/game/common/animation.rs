use bevy::prelude::*;
use crate::game::common::layer2d::Layer2d;

pub(super) fn build_animation(app: &mut App) {
    app.add_systems(PostUpdate, animate_sprite);
}

const FIRST_INDEX: usize = 0;

#[derive(Bundle)]
pub struct AnimationBundle {
    pub indices: AnimationIndices,
    pub timer: AnimationTimer,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub layer: Layer2d,
}

impl AnimationBundle {
    pub fn new(position: Vec2, layer: Layer2d, duration: f32, frames: usize, texture_atlas: Handle<TextureAtlas>) -> Self {
        let fps = duration / frames as f32;
        Self {
            indices: AnimationIndices {
                first: FIRST_INDEX,
                last: frames - 1,
            },
            timer: AnimationTimer(AnimationTimer::get_timer(fps)),
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas,
                sprite: TextureAtlasSprite::new(FIRST_INDEX),
                transform: Transform::from_translation(layer.vec2_to_vec3(position)),
                ..default()
            },
            layer,
        }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

impl AnimationIndices {
    fn new(fps: usize) -> Self {
        Self { first: FIRST_INDEX, last: Self::get_last_frame(fps) }
    }
    pub fn setup(&mut self, frames: usize) { self.last = Self::get_last_frame(frames); }
    fn get_last_frame(frames: usize) -> usize { frames - 1 }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn reset(&mut self, fps: f32) {
        self.0 = Self::get_timer(fps);
    }

    fn get_timer(fps: f32) -> Timer {
        Timer::from_seconds(fps, TimerMode::Repeating)
    }
}

fn animate_sprite(mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}