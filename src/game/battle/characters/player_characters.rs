use std::ops::Deref;
use bevy::prelude::*;
use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::controller_direct::ControllerDirect;
use crate::game::battle::characters::controller_indirect::{ControllerIndirect, Directive};
use crate::game::battle::characters::non_player_characters::NonPlayerCharacter;
use crate::game::battle::characters::selection_mark::SelectionMarkBundle;
use crate::game::common::cursor_collider::CursorCollider;
use crate::game::input::indirect_input::IndirectInputCursor;
use crate::game::game_mode::GameMode;
use crate::game::registry::{AttackRange, CharacterOrigin};

pub(super) fn build_player_characters(app: &mut App) {
    app.add_systems(OnEnter(GameMode::Battle), spawn_player_characters)
        .add_systems(Update, handle_indirect_selection_input)
        .add_systems(Update, handle_indirect_action_input);
}

#[derive(Component)]
pub struct PlayerCharacter(AttackRange);

fn spawn_player_characters(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, origin| {
        commands.spawn((
            CharacterBundle::new(origin, position, &asset_server, &mut texture_atlases),
            PlayerCharacter(AttackRange::Regular),
            ControllerDirect::default(),
            ControllerIndirect::default()
        )).with_children(|parent| {
            parent.spawn(SelectionMarkBundle::new(&asset_server.deref()));
        });
    };
    const SHIFT: f32 = 100.;
    do_spawn(Vec2::new(0., SHIFT), CharacterOrigin::Red);
    // do_spawn(Vec2::new(0., -SHIFT),CharacterOrigin::Candy);
    // do_spawn(Vec2::new(SHIFT, 0.), CharacterOrigin::Knife);
    // do_spawn(Vec2::new(-SHIFT, 0.), CharacterOrigin::Rose);
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

fn handle_indirect_action_input(mut player_q: Query<(&mut ControllerIndirect, &ControllerDirect, &PlayerCharacter)>,
                                npc_q: Query<(&CursorCollider, Entity), With<NonPlayerCharacter>>,
                                cursor: Res<IndirectInputCursor>) {
    if cursor.do_action().is_none() {
        return;
    }

    let target = *cursor.position();

    let mut npc_under_action = None;
    for (collider, entity) in &npc_q {
        if *collider.hovered() {
            npc_under_action = Some(entity);
            // let's avoid "multi-command per click"
            break;
        }
    }

    for (mut indirect, direct, player_character) in &mut player_q {
        if direct.active() || !indirect.selected {
            continue;
        }

        if let Some(npc) = npc_under_action {
            indirect.set_directive(Directive::Attack(npc, player_character.0))
        } else if !cursor.on_collider {
            indirect.set_directive(Directive::MoveTo(target, false))
        }
    }
}