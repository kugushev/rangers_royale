use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::ecs::battle::characters::animations::CharacterAnimationBundle;
use crate::ecs::scenes::GameScene;
use crate::registry::character_animations_paths::YOUNG_HERO;

pub(super) fn build_player_characters(app: &mut App, scene: GameScene) {
    app.add_systems(OnEnter(scene), spawn_player_characters);
}

#[derive(Component)]
struct PlayerCharacter;

fn spawn_player_characters(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let start_position = Vec2::new(0.0, 0.0);
    commands.spawn((
        PlayerCharacter,
        CharacterAnimationBundle::new(start_position, &YOUNG_HERO, &asset_server, &mut texture_atlases),
    ));
}