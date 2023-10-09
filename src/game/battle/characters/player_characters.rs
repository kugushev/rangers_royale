use std::ops::Deref;
use bevy::prelude::*;
use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::character_animations_paths::{FEM_CANDY, FEM_RED, FEM_ROSE};
use crate::game::battle::characters::controller_direct::ControllerDirect;
use crate::game::battle::characters::controller_indirect::{ControllerIndirect, Directive};
use crate::game::battle::characters::selection_mark::SelectionMarkBundle;
use crate::game::common::cursor_collider::CursorCollider;
use crate::game::input::indirect_input::IndirectInputCursor;
use crate::game::game_mode::GameMode;

pub(super) fn build_player_characters(app: &mut App) {
    app.add_systems(OnEnter(GameMode::Battle), spawn_player_characters)
        .add_systems(Update, handle_indirect_selection_input)
        .add_systems(Update, handle_indirect_action_input);
}

#[derive(Component)]
pub struct PlayerCharacter;

fn spawn_player_characters(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, paths| {
        commands.spawn((
            CharacterBundle::new(position, paths, &asset_server, &mut texture_atlases),
            PlayerCharacter,
            ControllerDirect::default(),
            ControllerIndirect::default()
        )).with_children(|parent| {
            parent.spawn(SelectionMarkBundle::new(&asset_server.deref()));
        });
    };
    const SHIFT: f32 = 100.;
    do_spawn(Vec2::new(0., SHIFT), &FEM_RED);
    do_spawn(Vec2::new(0., -SHIFT), &FEM_CANDY);
    // do_spawn(Vec2::new(SHIFT, 0.), &FEM_KNIFE);
    do_spawn(Vec2::new(-SHIFT, 0.), &FEM_ROSE);
}

fn handle_indirect_selection_input(mut query: Query<(&CursorCollider, &mut ControllerIndirect, &ControllerDirect), With<PlayerCharacter>>, cursor: Res<IndirectInputCursor>) {
    if cursor.do_select().is_none() {
        return;
    }

    for (collider, mut indirect, direct) in &mut query {
        if direct.active() {
            continue;
        }

        indirect.selected = *collider.hovered();
    }
}

fn handle_indirect_action_input(mut query: Query<(&mut ControllerIndirect, &ControllerDirect), With<PlayerCharacter>>, cursor: Res<IndirectInputCursor>) {
    if cursor.do_action().is_none() {
        return;
    }

    let target = *cursor.position();

    for (mut indirect, direct) in &mut query {
        if direct.active() || !indirect.selected {
            continue;
        }

        indirect.set_directive(Directive::MoveTo(target));
    }
}