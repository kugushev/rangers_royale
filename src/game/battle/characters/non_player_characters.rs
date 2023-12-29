use bevy::prelude::*;

use crate::game::battle::characters::CharacterBundle;

use crate::game::battle::characters::selection_mark::SelectionMarkBundle;
use crate::game::game_mode::GameMode;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_non_player_characters(app: &mut App) {
    app.add_systems(OnEnter(GameMode::Battle), spawn_dummy);
}

#[derive(Component, Default)]
pub struct NonPlayerCharacter;

fn spawn_dummy(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, origin| {
        commands.spawn((
            CharacterBundle::new(origin, position, &asset_server, &mut texture_atlases),
            NonPlayerCharacter::default()
        )).with_children(|parent| {
            parent.spawn(SelectionMarkBundle::new(&asset_server));
        });
    };

    do_spawn(Vec2::new(300., 0.), CharacterOrigin::Knife);
}