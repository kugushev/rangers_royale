use bevy::prelude::*;
use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::character_animation::CharacterAnimationBundle;
use crate::game::battle::characters::character_animations_paths::{FEM_CANDY, FEM_KNIFE, FEM_RED, FEM_ROSE};
use crate::game::players::host_cursor::HostCursor;
use crate::game::common::moving::MoveCommand;
use crate::game::game_mode::GameMode;
use crate::game::players::actors_inputs::{ActorInput, ActorsInputs};
use crate::game::utils::Vec3Ex;

pub(super) fn build_player_characters(app: &mut App) {
    app.add_systems(OnEnter(GameMode::Battle), spawn_player_characters)
        .add_systems(PreUpdate, toggle_inputs_per_character.run_if(in_state(GameMode::Battle)))
        .add_systems(PreUpdate, move_character.after(toggle_inputs_per_character).run_if(in_state(GameMode::Battle)));
}

#[derive(Component, Default)]
struct PlayerCharacter {
    direct_input_id: Option<usize>,
}

fn spawn_player_characters(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, paths| {
        commands.spawn((
            CharacterBundle::default(),
            PlayerCharacter::default(),
            CharacterAnimationBundle::new(position, paths, &asset_server, &mut texture_atlases),
            MoveCommand::default()
        ));
    };
    const SHIFT: f32 = 100.;
    do_spawn(Vec2::new(0., SHIFT), &FEM_RED);
    do_spawn(Vec2::new(0., -SHIFT), &FEM_CANDY);
    do_spawn(Vec2::new(SHIFT, 0.), &FEM_KNIFE);
    do_spawn(Vec2::new(-SHIFT, 0.), &FEM_ROSE);
}

fn toggle_inputs_per_character(mut query: Query<&mut PlayerCharacter>, actors_inputs: Res<ActorsInputs>) {
    // 'cleanup' characters from unplugged gamepads
    for mut character in &mut query {
        if let Some(direct_input_id) = character.direct_input_id {
            if actors_inputs.get(direct_input_id).is_none() {
                character.direct_input_id = None;
            }
        }
    }

    'actors: for (input_id, _) in actors_inputs.get_actors() {
        let mut candidate = None;
        for character in &mut query {
            if let Some(direct_input_id) = character.direct_input_id {
                if direct_input_id == *input_id {
                    continue 'actors;
                }
            } else {
                candidate = Some(character);
            }
        }

        if let Some(mut character) = candidate {
            character.direct_input_id = Some(*input_id);
        } else {
            warn!("Unable to add new gamepad {input_id}")
        }
    }
}

fn move_character(mut query: Query<(&mut MoveCommand, &PlayerCharacter, &Transform)>, actors_inputs: Res<ActorsInputs>, cursor: Res<HostCursor>) {
    let host_target = match cursor.action_command() {
        Some(c) => { Some(*cursor.position()) }
        None => { None }`
    };

    for (mut move_command, character, transform) in &mut query {
        if let Some(input_id) = character.direct_input_id {
            if let Some(input) = actors_inputs.get(input_id) {
                let mut position = transform.translation.to_vec2();
                position.x += input.horizontal();
                position.y += input.vertical();
                move_command.target = Some(position);
            } else {
                warn!("Input not found {input_id}")
            }
        } else {
            // don't override current command if there is no input
            if host_target.is_some() {
                move_command.target = host_target;
            }
        }
    }
}