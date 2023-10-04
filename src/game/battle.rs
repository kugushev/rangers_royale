use bevy::prelude::*;
use crate::game::battle::characters::build_characters;
use crate::game::battle::world::build_world;
use crate::game::game_mode::GameMode;

pub mod world;
mod characters;

pub(super) fn build_battle(app: &mut App) {
    app.add_systems(OnExit(GameMode::Battle), cleanup);
    build_world(app);
    build_characters(app);
}

#[derive(Component)]
pub struct BattleTag;

fn cleanup(mut query: Query<Entity, With<BattleTag>>, mut commands: Commands) {
    for entity in &mut query {
        commands.entity(entity).despawn();
    }
}
