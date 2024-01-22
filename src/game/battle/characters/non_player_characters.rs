use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;

use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::controllers::indirect::{ControllerIndirect, Directive, DirectiveSource};
use crate::game::battle::characters::controllers::indirect::ai::AiAlgorithm;
use crate::game::battle::characters::faction::Faction;
use crate::game::battle::characters::player_characters::PlayerCharacter;

use crate::game::battle::characters::selection_mark::SelectionMarkBundle;
use crate::game::game_mode::GameMode;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_non_player_characters(app: &mut App) {
    app.add_systems(OnEnter(GameMode::Battle), spawn_enemy);
}

#[derive(Component, Default)]
pub struct NonPlayerCharacter;

fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
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
    // do_spawn(Vec2::new(300. + SHIFT, 0.), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(300. - SHIFT, 0.), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(300., 0. + SHIFT), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(300., 0. - SHIFT), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(300. + SHIFT, 0. - SHIFT), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(300. + SHIFT, 0. + SHIFT), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(300. - SHIFT, 0. - SHIFT), CharacterOrigin::Orc);
    // do_spawn(Vec2::new(300. - SHIFT, 0. + SHIFT), CharacterOrigin::Orc);

    do_spawn(Vec2::new(600. + SHIFT, 0.), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600. - SHIFT, 0.), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600., 0. + SHIFT), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600., 0. - SHIFT), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600. + SHIFT, 0. - SHIFT), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600. + SHIFT, 0. + SHIFT), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600. - SHIFT, 0. - SHIFT), CharacterOrigin::Orc);
    do_spawn(Vec2::new(600. - SHIFT, 0. + SHIFT), CharacterOrigin::Orc);
}