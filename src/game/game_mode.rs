use bevy::prelude::*;

pub(super) fn build_game_mode(app: &mut App) {
    app.add_state::<GameMode>();
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameMode {
    #[default]
    MainMenu,
    Battle,
}