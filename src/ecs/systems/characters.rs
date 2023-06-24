use std::ops::{Deref, DerefMut};
use bevy::prelude::*;
use crate::ecs::components::{Character, CharacterOrder, CharacterOrdersHandle};
use crate::ecs::components::deck::{CharacterHand, Deck, HandSlot};

pub(crate) fn build_characters_systems(app: &mut App) {
    app.add_system(handle_orders)
        .add_system(refresh_hand);
}

const CHARACTER_SPEED: f32 = 100.0;

fn handle_orders(mut query: Query<(&mut CharacterOrdersHandle, &mut Transform)>, time: Res<Time>) {
    for (mut orders_handle, transform) in &mut query {
        let order_opt = orders_handle.get_mut();
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

const REFRESH_COOLDOWN: f32 = 1.0;

fn refresh_hand(mut query: Query<(&mut Deck, &mut CharacterHand)>, time: Res<Time>) {
    for (mut deck, mut hand) in &mut query {
        handle_hand_slot(deck.deref_mut(), time.deref(), &mut hand.q);
        handle_hand_slot(deck.deref_mut(), time.deref(), &mut hand.w);
        handle_hand_slot(deck.deref_mut(), time.deref(), &mut hand.e);
        handle_hand_slot(deck.deref_mut(), time.deref(), &mut hand.r);
    }

    fn handle_hand_slot(deck: &mut Deck, time: &Time, slot: &mut HandSlot) {
        if let HandSlot::Empty = slot {
            try_obtain(slot, deck);
        }

        if let HandSlot::Cooldown(cooldown) = slot {
            *cooldown += time.delta_seconds();
            if *cooldown >= REFRESH_COOLDOWN {
                if !try_obtain(slot, deck) {
                    *slot = HandSlot::Empty;
                }
            }
        }

        fn try_obtain(slot: &mut HandSlot, deck: &mut Deck) -> bool {
            if let Some(card) = deck.take() {
                *slot = HandSlot::Obtained(card);
                true
            } else { false }
        }
    }
}