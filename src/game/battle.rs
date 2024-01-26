use bevy::prelude::*;
use crate::game::battle::characters::build_characters;
use crate::game::battle::encounter::build_encounter;
use crate::game::game_mode::GameMode;

pub mod encounter;
pub mod characters;
pub mod value_objects;

pub(super) fn build_battle(app: &mut App) {
    app.add_systems(OnExit(GameMode::Battle), cleanup);
    build_encounter(app);
    build_characters(app);
}

#[derive(Component)]
pub struct BattleTag;

fn cleanup(mut query: Query<Entity, With<BattleTag>>, mut commands: Commands) {
    for entity in &mut query {
        commands.entity(entity).despawn();
    }
}
