pub mod animations;

use std::ops::{Deref, DerefMut};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::ecs::components::characters::{Character, CharacterOrder, CharacterOrdersHandle};
use crate::ecs::components::deck::{Card, Deck, HandSlot, SkillsHand};
use crate::ecs::components::deck::HandSlot::Obtained;
use crate::ecs::components::projectiles::MagicMissile;
use crate::ecs::components::ui::SkillKey;
use crate::ecs::systems::characters::animations::build_characters_animations;

pub(crate) fn build_characters_systems(app: &mut App) {
    app.add_system(handle_orders)
        .add_system(refresh_hand);

    build_characters_animations(app);
}

const CHARACTER_SPEED: f32 = 100.0;

fn handle_orders(mut query: Query<(&mut CharacterOrdersHandle, &mut Transform, &mut SkillsHand)>,
                 time: Res<Time>, mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<ColorMaterial>>) {
    for (mut orders_handle, transform, hand) in &mut query {
        let order_opt = orders_handle.get_mut();
        if let Some(order) = order_opt {
            let handled = handle_impl(order, transform, hand, &time, &mut commands, meshes.deref_mut(), materials.deref_mut());
            if handled {
                *order_opt = None;
            }
        }
    }

    fn handle_impl(order: &mut CharacterOrder, mut transform: Mut<Transform>, mut hand: Mut<SkillsHand>,
                   time: &Res<Time>,
                   commands: &mut Commands,
                   mut meshes: &mut Assets<Mesh>,
                   mut materials: &mut Assets<ColorMaterial>) -> bool {
        match order {
            CharacterOrder::MoveToPosition(target) => {
                if let Some(value) = handle_move_to_position(transform, time, target) {
                    return value;
                }
            }

            CharacterOrder::UseSkill(key, target) => {
                handle_use_skill(transform.deref(), hand, key, target, commands, meshes, materials);
                return true;
            }
        }
        false
    }
    fn handle_move_to_position(mut transform: Mut<Transform>, time: &Res<Time>, target: &mut Vec2) -> Option<bool> {
        let current = Character::translation_to_vec2(transform.deref());

        let move_length = time.delta_seconds() * CHARACTER_SPEED;
        let delta = *target - current;
        if delta.length() <= move_length {
            transform.translation = Character::vec2_to_translation(&target);
            return Some(true);
        } else {
            let direction = delta.normalize() * move_length;
            let new_position = current + direction;

            transform.translation = Character::vec2_to_translation(&new_position);
        }
        None
    }

    fn handle_use_skill(transform: &Transform, mut hand: Mut<SkillsHand>, key: &mut SkillKey,
                        target: &mut Vec2, commands: &mut Commands,
                        mut meshes: &mut Assets<Mesh>,
                        mut materials: &mut Assets<ColorMaterial>, ) {
        let slot = match key {
            SkillKey::Q => { &mut hand.q }
            SkillKey::W => { &mut hand.w }
            SkillKey::E => { &mut hand.e }
            SkillKey::R => { &mut hand.r }
        };

        if let Obtained(card) = slot {
            match card {
                Card::MagicMissile => {
                    commands.spawn((
                        MagicMissile {
                            start: Character::translation_to_vec2(transform),
                            finish: target.clone(),
                        },
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::PURPLE)),
                            transform: Transform::from_translation(transform.translation),
                            ..default()
                        }
                    ));
                }
            }

            *slot = HandSlot::Cooldown(0.0);
        }
    }
}

const REFRESH_COOLDOWN: f32 = 1.0;

fn refresh_hand(mut query: Query<(&mut Deck, &mut SkillsHand)>, time: Res<Time>) {
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