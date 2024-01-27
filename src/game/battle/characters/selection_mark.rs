use std::sync::Mutex;
use bevy::prelude::*;
use crate::game::battle::characters::controllers::direct::ControllerDirect;
use crate::game::battle::characters::controllers::indirect::{ControllerIndirect, DirectiveSource};
use crate::game::battle::characters::player_characters::PlayerCharacter;
use crate::game::common::layer2d::LAYER_SIZE;

pub(super) fn build_selection_mark(app: &mut App) {
    app.add_systems(Update, change_appearance);
}

#[derive(Bundle)]
pub struct SelectionMarkBundle {
    mark: SelectionMark,
    sprite: SpriteBundle,
}

static TEXTURE_CACHE: Mutex<Option<Handle<Image>>> = Mutex::new(None);

impl SelectionMarkBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        let mut cached_texture = TEXTURE_CACHE.lock().expect("Texture Cache is poisoned");
        let texture = cached_texture.get_or_insert_with(|| asset_server.load("my/Selector.png"));

        Self {
            mark: SelectionMark,
            sprite: SpriteBundle {
                texture: texture.clone_weak(),
                transform: Transform::from_xyz(0., 0., -1. * LAYER_SIZE),
                visibility: Visibility::Hidden,
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct SelectionMark;

fn change_appearance(mut query: Query<(&mut Sprite, &mut Visibility, &Parent), With<SelectionMark>>,
                     player_parent_query: Query<(&ControllerIndirect, &ControllerDirect), With<PlayerCharacter>>) {
    for (mut sprite, mut visibility, parent) in &mut query {
        *visibility = Visibility::Hidden;
        if let Ok((indirect, direct)) = player_parent_query.get(parent.get()) {
            *visibility = Visibility::Inherited;
            sprite.color = if direct.active() {
                Color::LIME_GREEN
            } else {
                let alpha = match indirect.source() {
                    DirectiveSource::PlayerInput { selected: true } => 1.,
                    DirectiveSource::PlayerInput { selected: false } => 0.1,
                    DirectiveSource::Ai(_) => 0.,
                };
                Color::default().with_a(alpha)
            }
        }
    }
}