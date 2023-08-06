use std::ops::Deref;
use bevy::prelude::*;
use crate::ecs::components::characters::Character;
use crate::ecs::components::projectiles::MagicMissile;

pub(crate) fn build_projectiles_systems(app: &mut App) {
    app.add_system(magic_missile);
}

fn magic_missile(mut query: Query<(&MagicMissile, &mut Transform, Entity)>, mut commands: Commands, time: Res<Time>) {
    const SPEED: f32 = 300.0;

    for (magic_missile, mut transform, entity) in &mut query {

        let current = Character::translation_to_vec2(transform.deref());
        let move_length = time.delta_seconds() * SPEED;
        let delta = magic_missile.finish - current;
        if delta.length() <= move_length {
            commands.entity(entity).despawn();
        } else {
            let direction = delta.normalize() * move_length;
            let new_position = current + direction;

            transform.translation = Character::vec2_to_translation(&new_position);
        }
    }
}