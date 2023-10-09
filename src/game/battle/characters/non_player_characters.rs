use bevy::prelude::*;
use crate::game::battle::characters::character_animations_paths::FEM_KNIFE;
use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::player_characters::PlayerCharacter;
use crate::game::battle::characters::selection_mark::SelectionMarkBundle;
use crate::game::common::cursor_collider::CursorCollider;
use crate::game::game_mode::GameMode;
use crate::game::input::indirect_input::IndirectInputCursor;

pub(super) fn build_non_player_characters(app: &mut App) {
    app.add_systems(OnEnter(GameMode::Battle), spawn_dummy)
        .add_systems(Update, handle_npc_under_action);
}

#[derive(Component, Default)]
struct NonPlayerCharacter;

fn spawn_dummy(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, paths| {
        commands.spawn((
            CharacterBundle::new(position, paths, &asset_server, &mut texture_atlases),
            NonPlayerCharacter::default(),
        )).with_children(|parent| {
            parent.spawn(SelectionMarkBundle::new(&asset_server));
        });
    };

    do_spawn(Vec2::new(300., 0.), &FEM_KNIFE);
}

fn handle_npc_under_action(mut query: Query<(&CursorCollider, &mut NonPlayerCharacter, Entity)>,
                           mut player_q: Query<&mut PlayerCharacter>, cursor: Res<IndirectInputCursor>) {
    if cursor.do_action().is_none() {
        return;
    }

    // todo: add directive issue here
    // todo: how to deal with race with MoveTo directive

    // let mut npc_under_action = None;
    // for (collider, mut npc, entity) in &mut query {
    //     if *collider.hovered() {
    //         npc_under_action = Some(entity);
    //
    //         // let's avoid "multi-command per click"
    //         break;
    //     }
    // }

    // if let Some(npc) = npc_under_action {
    //     for player_character in &mut player_q {
    //         match player_character.controller() {
    //             InputType::IndirectIfSelected(true) => {}
    //             _ => continue
    //         }
    //
    //
    //
    //
    //     }
    // }
}