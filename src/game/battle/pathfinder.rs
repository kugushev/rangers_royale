use bevy::prelude::*;
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::common::obstacle::Obstacle;

pub(super) fn build_pathfinder(app: &mut App) {
    app.insert_resource(Pathfinder)
        .add_systems(First, build_graph);
}

#[derive(Resource)]
pub struct Pathfinder;

fn build_graph(mut pathfinder: ResMut<Pathfinder>, mut query: Query<(&CharacterState, &GlobalTransform, &Obstacle)>) {
    for (state, transform, obstacle) in &mut query {

    }
}