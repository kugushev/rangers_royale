use bevy::prelude::*;
use crate::game::battle::build_battle;
use crate::game::common::build_common;
use crate::game::game_mode::build_game_mode;
use crate::game::main_menu::build_main_menu;
use crate::game::input::build_players;

mod battle;
mod common;
pub mod utils;
mod game_mode;
mod main_menu;
pub mod input;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        build_game_mode(app);
        build_common(app);
        build_battle(app);
        build_main_menu(app);
        build_players(app);
    }
}




