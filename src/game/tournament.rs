use bevy::prelude::*;
use crate::game::battle::encounter::Encounter;
use crate::game::game_mode::GameMode;

pub(super) fn build_tournament(app: &mut App){
    app.insert_resource(Tournament::default())
        .add_systems(OnEnter(GameMode::Tournament), next_round);
}

#[derive(Resource, Default)]
pub struct Tournament {
    pub money: u32,
    pub xp: u32,
    pub win: u32,
    pub loose: u32
}

impl Tournament {
    pub fn is_chamption(&self) -> bool {
        self.win >= 10
    }

    pub fn is_game_over(&self) -> bool {
        self.loose >= 3
    }
}

fn next_round(mut tournament: Res<Tournament>, mut encounter: ResMut<Encounter>, mut game_mode: ResMut<NextState<GameMode>>) {
    println!("Tournament: {}$ {}XP", tournament.money, tournament.xp);
    game_mode.set(GameMode::Battle);
}