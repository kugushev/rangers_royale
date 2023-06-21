use bevy::prelude::*;


#[derive(Component)]
pub(crate) struct Player;

#[derive(Bundle)]
pub(crate) struct Character {
    pub sprite: SpriteBundle,
    pub orders_handle: CharacterOrdersHandle
}

impl Character {
    pub fn vec2_to_translation(position: &Vec2) -> Vec3 {
        Vec3::new(position.x, position.y, 2.0)
    }

    pub fn translation_to_vec2(transform: &Transform) -> Vec2 {
        Vec2::new(transform.translation.x, transform.translation.y)
    }
}

#[derive(Component, Default)]
pub(crate) struct CharacterOrdersHandle {
    pub order: Option<CharacterOrder>,
}

pub(crate) enum CharacterOrder {
    MoveToPosition(Vec2),
}

