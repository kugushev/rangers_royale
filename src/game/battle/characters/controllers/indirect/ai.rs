use bevy::prelude::*;
use crate::game::battle::characters::arms::Arms;
use crate::game::battle::characters::controllers::indirect::{ControllerIndirect, Directive, DirectiveSource};
use crate::game::battle::characters::faction::Faction;

pub(super) fn build_ai(app: &mut App) {
    app.add_systems(Update, act_simple);
}

pub enum AiAlgorithm {
    Simple
}

fn act_simple(mut query: Query<(&mut ControllerIndirect, &GlobalTransform, &Arms, &Faction, Entity)>,
              other_q: Query<(&GlobalTransform, &Faction, Entity)>) {
    for (mut controller, transform, arms, faction, entity) in &mut query {
        match &mut controller.source {
            DirectiveSource::Ai(AiAlgorithm::Simple) => {}
            _ => continue,
        };

        if let Some(Directive::Attack(..)) = controller.directive() {
            continue;
        }

        // find nearest
        let mut nearest_opponent: Option<(f32, Entity)> = None;
        for (other_transform, other_faction, other_entity) in &other_q {
            if other_entity == entity {
                continue;
            }

            if !other_faction.is_rival(faction) {
                continue;
            }

            let distance = other_transform.translation().distance(transform.translation());

            if let Some((prev_dist, _)) = nearest_opponent {
                if prev_dist >= distance {
                    continue;
                }
            }

            nearest_opponent = Some((distance, other_entity));
        }

        let target = match nearest_opponent {
            None => { continue; }
            Some((_, t)) => t,
        };

        controller.set_directive(Directive::Attack(target, *arms));
    }
}