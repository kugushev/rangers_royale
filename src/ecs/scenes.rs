use bevy::prelude::*;

pub(super) fn build_scenes(app: &mut App) {
    app.add_state::<GameScene>();
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameScene {
    #[default]
    Battle
}