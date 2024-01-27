mod game_over;
mod champion;

use bevy::prelude::*;
use crate::game::game_mode::champion::build_champion;
use crate::game::game_mode::game_over::build_game_over;

pub(super) fn build_game_mode(app: &mut App) {
    app.add_state::<GameMode>();

    build_game_over(app);
    build_champion(app);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameMode {
    MainMenu,
    #[default]
    Tournament,
    Battle,
    GameOver,
    Chamption
}