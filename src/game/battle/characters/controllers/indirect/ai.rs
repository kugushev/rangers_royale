use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;
use crate::game::battle::characters::controllers::direct::ControllerDirect;
use crate::game::battle::characters::controllers::indirect::{ControllerIndirect, Directive, DirectiveSource};
use crate::game::battle::characters::non_player_characters::NonPlayerCharacter;
use crate::game::battle::characters::player_characters::PlayerCharacter;

pub(super) fn build_ai(app: &mut App) {
    app.add_systems(Update, act_simple);
}

pub enum AiAlgorithm {
    Simple
}

fn act_simple(mut query: Query<(&mut ControllerIndirect, &GlobalTransform, &Arms), (With<NonPlayerCharacter>, Without<ControllerDirect>)>,
              player_q: Query<(&GlobalTransform, Entity), With<PlayerCharacter>>) {
    for (mut controller, transform, arms) in &mut query {
        match &mut controller.source {
            DirectiveSource::Ai(AiAlgorithm::Simple) => {},
            _ => continue,
        };

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