use bevy::ecs::bundle::DynamicBundle;
use bevy::prelude::*;
use crate::game::battle::characters::Character;
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

fn change_appearance(mut query: Query<(&mut Sprite, &Parent), With<SelectionMark>>, mut parent_query: Query<&PlayerCharacter>) {
    for (mut sprite, parent) in &mut query {
        if let Ok(parent_character) = parent_query.get(parent.get()) {
            sprite.color = match parent_character.direct_input_id() {
                Some(_) => Color::LIME_GREEN,
                None => Color::default(),
            };
        }
    }
}