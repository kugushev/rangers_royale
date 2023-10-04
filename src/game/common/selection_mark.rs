use bevy::prelude::*;
use crate::game::common::layer2d::LAYER_SIZE;

pub(super) fn build_selection_mark(app: &mut App) {}

#[derive(Bundle)]
pub struct SelectionMarkBundle {
    mark: SelectionMark,
    sprite: SpriteBundle,
}

impl SelectionMarkBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            mark: SelectionMark,
            sprite: SpriteBundle {
                texture: asset_server.load("my/Selector.png"),
                transform: Transform::from_xyz(0., 0., -1. * LAYER_SIZE),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct SelectionMark;