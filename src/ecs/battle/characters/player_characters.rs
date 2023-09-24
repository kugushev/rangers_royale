use bevy::prelude::*;
use crate::ecs::battle::characters::{Character, CharacterBundle};
use crate::ecs::battle::characters::character_animation::CharacterAnimationBundle;
use crate::ecs::scenes::GameScene;
use crate::registry::character_animations_paths::{FEM_CANDY, FEM_KNIFE, FEM_RED, FEM_ROSE};
use crate::ecs::common::moving::MoveCommand;
use crate::ecs::common::player_input::{CursorPosition, PlayerInput};
use crate::utils::Vec3toVec2;

pub(super) fn build_player_characters(app: &mut App, scene: GameScene) {
    app.add_systems(OnEnter(scene), spawn_player_characters)
        .add_systems(Update, (move_player, cursor_hover).run_if(in_state(scene)));
}

#[derive(Component)]
struct PlayerCharacter;

fn spawn_player_characters(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, paths| {
        commands.spawn((
            CharacterBundle::default(),
            PlayerCharacter,
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

fn move_player(mut query: Query<&mut MoveCommand, With<PlayerCharacter>>, player_input: Res<PlayerInput>) {
    let target = match player_input.action_command() {
        Some(c) => { *c }
        None => { return; }
    };

    for mut move_command in &mut query {
        move_command.target = Some(target);
    }
}

fn cursor_hover(mut query: Query<&GlobalTransform, With<PlayerCharacter>>, cursor: Res<CursorPosition>) {
    for transform in &query {
        if transform.translation().to_vec2().distance(cursor.0) < 100.0 {
            println!("Hover")
        }
    }
}