use std::ops::Deref;
use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;
use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::controllers::direct::ControllerDirect;
use crate::game::battle::characters::controllers::indirect::{ControllerIndirect, DirectiveSource};
use crate::game::battle::characters::faction::Faction;
use crate::game::battle::characters::selection_mark::SelectionMarkBundle;
use crate::game::game_mode::GameMode;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_player_characters(app: &mut App) {
    app.add_systems(OnEnter(GameMode::Battle), spawn_player_characters);
}

#[derive(Component)]
pub struct PlayerCharacter;

fn spawn_player_characters(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, origin| {
        commands.spawn((
            CharacterBundle::new(origin, Faction::Player, position, &asset_server, &mut texture_atlases),
            PlayerCharacter,
            ControllerDirect::default(),
            ControllerIndirect::new(DirectiveSource::PlayerInput { selected: false }),
            Arms::Regular
        )).with_children(|parent| {
            parent.spawn(SelectionMarkBundle::new(&asset_server.deref()));
        });
    };
    const SHIFT: f32 = 100.;
    do_spawn(Vec2::new(0., SHIFT), CharacterOrigin::Red);
    do_spawn(Vec2::new(0., -SHIFT), CharacterOrigin::Candy);
    do_spawn(Vec2::new(SHIFT, 0.), CharacterOrigin::Knife);
    do_spawn(Vec2::new(-SHIFT, 0.), CharacterOrigin::Rose);
}