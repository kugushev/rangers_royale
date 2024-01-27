pub mod encounter_map;

use std::ops::Deref;
use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;
use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::controllers::direct::ControllerDirect;
use crate::game::battle::characters::controllers::indirect::{ControllerIndirect, DirectiveSource};
use crate::game::battle::characters::controllers::indirect::ai::AiAlgorithm;
use crate::game::battle::characters::faction::Faction;
use crate::game::battle::characters::non_player_characters::NonPlayerCharacter;
use crate::game::battle::characters::player_characters::PlayerCharacter;
use crate::game::battle::characters::selection_mark::SelectionMarkBundle;
use crate::game::battle::encounter::encounter_map::build_encounter_map;
use crate::game::game_mode::GameMode;
use crate::game::registry::CharacterOrigin;
use crate::game::tournament::Tournament;

pub(super) fn build_encounter(app: &mut App) {
    build_encounter_map(app);

    app.insert_resource(Encounter::default());

    app.add_systems(Update, finish_check.run_if(in_state(GameMode::Battle)))
        .add_systems(OnEnter(GameMode::Battle), spawn_player_characters)
        .add_systems(OnExit(GameMode::Battle), despawn_player_characters)
        .add_systems(OnEnter(GameMode::Battle), spawn_enemies)
        .add_systems(OnExit(GameMode::Battle), despawn_enemies);
}

#[derive(Resource, Default)]
pub struct Encounter {}

fn finish_check(player_chars_q: Query<(), With<PlayerCharacter>>, enemy_chars_q: Query<(), With<NonPlayerCharacter>>, mut game_mode: ResMut<NextState<GameMode>>, mut tournament: ResMut<Tournament>) {
    let mut player_chars = 0;
    for _ in &player_chars_q {
        player_chars += 1;
    }

    let mut enemy_chars = 0;
    for _ in &enemy_chars_q {
        enemy_chars += 1;
    }

    if player_chars == 0 || enemy_chars == 0 {
        if enemy_chars == 0 {
            tournament.money += 1;
            tournament.win += 1;
        } else {
            tournament.loose += 1;
        }

        tournament.xp += 4 - player_chars;

        let next_mode = if tournament.is_chamption() {
            GameMode::Chamption
        } else if tournament.is_game_over() {
            GameMode::GameOver
        } else {
            GameMode::Tournament
        };

        game_mode.set(next_mode)
    }
}

fn spawn_player_characters(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, origin| {
        commands.spawn((
            CharacterBundle::new(origin, Faction::Player, position, &asset_server, &mut texture_atlases),
            PlayerCharacter,
            ControllerDirect::default(),
            // ControllerIndirect::new(DirectiveSource::PlayerInput { selected: false }),
            ControllerIndirect::new(DirectiveSource::Ai(AiAlgorithm::Simple)),
            Arms::Regular
        )).with_children(|parent| {
            parent.spawn(SelectionMarkBundle::new(&asset_server.deref()));
        });
    };
    const SHIFT: f32 = 100.;
    // do_spawn(Vec2::new(-300., SHIFT), CharacterOrigin::Red);
    // do_spawn(Vec2::new(-300., -SHIFT), CharacterOrigin::Candy);
    // do_spawn(Vec2::new(-300. + SHIFT, 0.), CharacterOrigin::Knife);
    // do_spawn(Vec2::new(-300. - SHIFT, 0.), CharacterOrigin::Rose);

    do_spawn(Vec2::new(-600., SHIFT), CharacterOrigin::Red);
    do_spawn(Vec2::new(-600., -SHIFT), CharacterOrigin::Candy);
    do_spawn(Vec2::new(-600. + SHIFT, 0.), CharacterOrigin::Knife);
    do_spawn(Vec2::new(-600. - SHIFT, 0.), CharacterOrigin::Rose);
}


fn despawn_player_characters(mut commands: Commands, query: Query<Entity, With<PlayerCharacter>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, origin| {
        commands.spawn((
            CharacterBundle::new(origin, Faction::Enemy, position, &asset_server, &mut texture_atlases),
            NonPlayerCharacter::default(),
            ControllerIndirect::new(DirectiveSource::Ai(AiAlgorithm::Simple)),
            Arms::Regular
        )).with_children(|parent| {
            parent.spawn(SelectionMarkBundle::new(&asset_server));
        });
    };

    const SHIFT: f32 = 100.;
    do_spawn(Vec2::new(600. + SHIFT, 0.), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600. - SHIFT, 0.), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600., 0. + SHIFT), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600., 0. - SHIFT), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(600. + SHIFT, 0. - SHIFT), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(600. + SHIFT, 0. + SHIFT), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(600. - SHIFT, 0. - SHIFT), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(600. - SHIFT, 0. + SHIFT), CharacterOrigin::Orc);
}

fn despawn_enemies(mut commands: Commands, query: Query<Entity, With<NonPlayerCharacter>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}