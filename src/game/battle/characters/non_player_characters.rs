use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;

use crate::game::battle::characters::CharacterBundle;
use crate::game::battle::characters::controller_indirect::{ControllerIndirect, Directive};
use crate::game::battle::characters::faction::Faction;
use crate::game::battle::characters::player_characters::PlayerCharacter;

use crate::game::battle::characters::selection_mark::SelectionMarkBundle;
use crate::game::game_mode::GameMode;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_non_player_characters(app: &mut App) {
    app.add_systems(OnEnter(GameMode::Battle), spawn_enemy)
        .add_systems(Update, handle_ai_act);
    ;
}

#[derive(Component, Default)]
pub struct NonPlayerCharacter;

fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let mut do_spawn = |position, origin| {
        commands.spawn((
            CharacterBundle::new(origin, Faction::Enemy, position, &asset_server, &mut texture_atlases),
            NonPlayerCharacter::default(),
            ControllerIndirect::default(),
            Arms::Regular
        )).with_children(|parent| {
            parent.spawn(SelectionMarkBundle::new(&asset_server));
        });
    };

    do_spawn(Vec2::new(300., 0.), CharacterOrigin::Orc);
}

fn handle_ai_act(mut query: Query<(&mut ControllerIndirect, &GlobalTransform, &Arms), With<NonPlayerCharacter>>,
                 player_q: Query<(&GlobalTransform, Entity), With<PlayerCharacter>>) {
    for (mut controller, transform, arms) in &mut query {
        if let Some(Directive::Attack(..)) = controller.directive() {
            continue;
        }

        // find nearest
        let mut nearest_player: Option<(f32, Entity)> = None;
        for (player_transform, player_entity) in &player_q {
            let distance = player_transform.translation().distance(transform.translation());

            if let Some((prev_dist, _)) = nearest_player {
                if prev_dist >= distance {
                    continue;
                }
            }

            nearest_player = Some((distance, player_entity));
        }

        let target = match nearest_player {
            None => { continue; }
            Some((_, t)) => t,
        };

        controller.set_directive(Directive::Attack(target, *arms));
    }
}