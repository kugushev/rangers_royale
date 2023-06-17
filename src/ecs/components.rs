use bevy::prelude::{Bundle, Component, SpriteBundle};


#[derive(Component)]
pub(crate) struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Bundle)]
pub(crate) struct Character {
    pub position: Position,
    pub sprite: SpriteBundle,
}