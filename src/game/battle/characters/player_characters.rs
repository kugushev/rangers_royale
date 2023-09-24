use bevy::prelude::*;
use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::character_animation::CharacterAnimationBundle;
use crate::game::battle::characters::character_animations_paths::{FEM_CANDY, FEM_KNIFE, FEM_RED, FEM_ROSE};
use crate::game::common::game_cursor::GameCursor;
use crate::game::scenes::GameScene;
use crate::game::common::moving::MoveCommand;
use crate::game::common::player_input::PlayerInput;
use crate::game::utils::Vec3Ex;

pub(super) fn build_player_characters(app: &mut App, scene: GameScene) {
    app.add_systems(OnEnter(scene), spawn_player_characters)
        .add_systems(Update, move_player.run_if(in_state(scene)));
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

fn move_player(mut query: Query<&mut MoveCommand, With<PlayerCharacter>>, player_input: Res<PlayerInput>, cursor: Res<GameCursor>) {
    let target = match player_input.action_command() {
        Some(c) => { *cursor.position() }
        None => { return; }
    };

    for mut move_command in &mut query {
        move_command.target = Some(target);
    }
}