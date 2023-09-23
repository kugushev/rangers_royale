use bevy::prelude::*;
use crate::ecs::battle::characters::{Character, CharacterBundle};
use crate::ecs::battle::characters::character_animation::CharacterAnimationBundle;
use crate::ecs::scenes::GameScene;
use crate::registry::character_animations_paths::YOUNG_HERO;
use crate::ecs::common::moving::MoveCommand;
use crate::ecs::common::player_input::PlayerInput;

pub(super) fn build_player_characters(app: &mut App, scene: GameScene) {
    app.add_systems(OnEnter(scene), spawn_player_characters)
        .add_systems(Update, move_player.run_if(in_state(scene)));
}

#[derive(Component)]
struct PlayerCharacter;

fn spawn_player_characters(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    commands.spawn((
        CharacterBundle::default(),
        PlayerCharacter,
        CharacterAnimationBundle::new(Vec2::ZERO, &YOUNG_HERO, &asset_server, &mut texture_atlases),
        MoveCommand::default()
    ));
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