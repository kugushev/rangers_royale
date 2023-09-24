use bevy::prelude::*;
use crate::game::battle::characters::build_characters;
use crate::game::battle::world::build_world;
use crate::game::scenes::GameScene;

mod world;
mod characters;

pub(super) fn build_battle(app: &mut App) {
    let scene = GameScene::Battle;
    build_world(app, scene.clone());
    build_characters(app, scene.clone());
}
