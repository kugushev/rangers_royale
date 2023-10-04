use bevy::prelude::*;
use crate::game::common::layer2d::Layer2d;

pub(super) fn build_animation(app: &mut App) {
    app.add_systems(Update, animate_sprite);
}

const FIRST_INDEX: usize = 0;

#[derive(Bundle)]
pub struct AnimationBundle {
    pub indices: AnimationIndices,
    pub timer: AnimationTimer,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub layer: Layer2d
}

impl AnimationBundle {
    pub fn new(position: Vec2, layer: Layer2d, fps: usize, texture_atlas: Handle<TextureAtlas>) -> Self {
        Self {
            indices: AnimationIndices {
                first: FIRST_INDEX,
                last: fps - 1,
            },
            timer: AnimationTimer(Timer::from_seconds(1.0 / fps as f32, TimerMode::Repeating)),
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
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
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