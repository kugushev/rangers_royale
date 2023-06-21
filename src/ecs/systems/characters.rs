use std::ops::Deref;
use bevy::prelude::*;
use crate::ecs::components::{Character, CharacterOrder, CharacterOrdersHandle};

pub(crate) fn build_characters_systems(app: &mut App) {
    app.add_system(handle_orders);
}

const CHARACTER_SPEED: f32 = 100.0;

fn handle_orders(mut query: Query<(&mut CharacterOrdersHandle, &mut Transform)>, time: Res<Time>) {
    for (mut orders_handle, transform) in &mut query {
        let order_opt = &mut orders_handle.order;
        if let Some(order) = order_opt {
            let handled = handle_impl(order, transform, &time);
            if handled {
                *order_opt = None;
            }
        }
    }

    fn handle_impl(order: &mut CharacterOrder, mut transform: Mut<Transform>, time: &Res<Time>) -> bool {
        match order {
            CharacterOrder::MoveToPosition(target) => {
                let current = Character::translation_to_vec2(transform.deref());

                let move_length = time.delta_seconds() * CHARACTER_SPEED;
                let delta = *target - current;
                if delta.length() <= move_length {
                    transform.translation = Character::vec2_to_translation(&target);
                    return true;
                } else {
                    let direction = delta.normalize() * move_length;
                    let new_position = current + direction;

                    transform.translation = Character::vec2_to_translation(&new_position);
                }
            }
        }
        false
    }
}