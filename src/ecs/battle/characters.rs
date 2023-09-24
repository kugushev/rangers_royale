mod player_characters;
mod character_animation;
mod position_tracker;

use bevy::prelude::*;
use crate::ecs::battle::characters::character_animation::build_character_animation;
use crate::ecs::battle::characters::player_characters::build_player_characters;
use crate::ecs::battle::characters::position_tracker::{build_position_tracking, PositionTracker};
use crate::ecs::scenes::GameScene;
use crate::utils::Z_LAYER;

pub(super) fn build_characters(app: &mut App, scene: GameScene) {
    app.add_systems(PostUpdate, y_sort.run_if(in_state(scene)));

    build_position_tracking(app);
    build_character_animation(app, scene);
    build_player_characters(app, scene);
}

#[derive(Bundle, Default)]
pub struct CharacterBundle {
    character: Character,
    position_tracker: PositionTracker,
}

#[derive(Component, Default)]
pub struct Character;


fn y_sort(mut q: Query<(&mut Transform), With<Character>>) {
    for mut tf in q.iter_mut() {
        tf.translation.z = Z_LAYER - (1.0 / (1.0 + (2.0f32.powf(-0.01 * tf.translation.y))));
    }
}