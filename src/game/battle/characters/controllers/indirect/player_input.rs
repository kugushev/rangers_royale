use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;
use crate::game::battle::characters::controllers::direct::{ControllerDirect, is_direct_active};
use crate::game::battle::characters::controllers::indirect::{ControllerIndirect, Directive, DirectiveSource};
use crate::game::battle::characters::non_player_characters::NonPlayerCharacter;
use crate::game::battle::characters::player_characters::PlayerCharacter;
use crate::game::common::cursor_collider::CursorCollider;
use crate::game::input::indirect_input::IndirectInputCursor;

pub(super) fn build_player_input(app: &mut App) {
    app.add_systems(Update, handle_indirect_selection_input)
        .add_systems(Update, handle_indirect_action_input);
}

fn handle_indirect_selection_input(mut query: Query<(&CursorCollider, &mut ControllerIndirect, Option<&ControllerDirect>), With<PlayerCharacter>>, cursor: Res<IndirectInputCursor>) {
    if cursor.do_select().is_none() {
        return;
    }

    for (collider, mut indirect, direct) in &mut query {
        if is_direct_active(direct) { continue; }

        let selected = match &mut indirect.source {
            DirectiveSource::PlayerInput { selected: s @ _ } => s,
            _ => continue,
        };


        *selected = *collider.hovered();
    }
}

fn handle_indirect_action_input(mut player_q: Query<(&mut ControllerIndirect, Option<&ControllerDirect>, &Arms), With<PlayerCharacter>>,
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

    for (mut indirect, direct, arms) in &mut player_q {
        if is_direct_active(direct) { continue; }

        if let DirectiveSource::PlayerInput { selected: true } = &indirect.source {
            if let Some(npc) = npc_under_action {
                indirect.set_directive(Directive::Attack(npc, *arms))
            } else if !cursor.on_collider {
                indirect.set_directive(Directive::MoveTo(target, false))
            }
        }
    }
}