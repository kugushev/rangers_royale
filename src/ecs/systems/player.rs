use std::ops::Deref;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::ecs::components::{Character, CharacterOrdersHandle, Player};
use crate::ecs::components::deck::{SkillsHand, Deck};
use crate::ecs::components::deck::Card::MagicMissile;
use crate::ecs::components::input::CursorPosition;
use crate::ecs::components::ui::SkillKey;
use crate::ecs::resources::WorldMap;

pub(crate) fn build_player_systems(app: &mut App) {
    app.add_startup_system(setup_player)
        .add_startup_system(setup_cursor_info)
        .add_system(handle_mouse_position)
        .add_system(handle_mouse_input)
        .add_system(handle_keyboard_input)
        .add_system(camera_move);
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = SpriteBundle {
        texture: asset_server.load("paid/player.png"),
        transform: Transform::from_translation(Character::vec2_to_translation(&Vec2::new(0.0, 0.0))),
        ..default()
    };

    commands.spawn((
        Character {
            sprite,
            orders_handle: default(),
        },
        Deck((0..42).map(|_| { MagicMissile }).collect()),
        SkillsHand::default(),
        Player
    ));
}

fn setup_cursor_info(mut commands: Commands) {
    commands.spawn(CursorPosition(Vec2::default()));
}

fn handle_mouse_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    cursor_q: Query<&mut CursorPosition>,
) {
    inner(windows, camera_q, cursor_q);

    fn inner(windows: Query<&Window, With<PrimaryWindow>>,
             camera_q: Query<(&GlobalTransform, &Camera)>,
             mut cursor_q: Query<&mut CursorPosition>) -> Option<()> {
        let cursor_position = windows.single().cursor_position()?;
        let (trans, cam) = camera_q.single();
        let world_position = cam.viewport_to_world_2d(trans, cursor_position)?;

        let mut cursor = cursor_q.single_mut();
        cursor.0 = world_position;
        Some(())
    }
}

fn handle_mouse_input(
    mouse_button_input: Res<Input<MouseButton>>,
    cursor_q: Query<&mut CursorPosition>,
    mut player_q: Query<&mut CharacterOrdersHandle, With<Player>>,
    world_map: Res<WorldMap>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let cursor_position = cursor_q.single();
    let mut orders_handle = player_q.single_mut();
    orders_handle.order_move_to_position(cursor_position.0, world_map.deref());
}

fn handle_keyboard_input(keyboard_input: Res<Input<KeyCode>>,
                         cursor_q: Query<&mut CursorPosition>,
                         mut player_q: Query<&mut CharacterOrdersHandle, With<Player>>,
) {
    let key = if keyboard_input.just_pressed(KeyCode::Q) {
        SkillKey::Q
    } else if keyboard_input.just_pressed(KeyCode::W) {
        SkillKey::W
    } else if keyboard_input.just_pressed(KeyCode::E) {
        SkillKey::E
    } else if keyboard_input.just_pressed(KeyCode::R) {
        SkillKey::R
    } else {
        return;
    };

    let cursor_position = cursor_q.single();
    let mut orders_handle = player_q.single_mut();
    orders_handle.order_use_skill(key, cursor_position.0);
}

fn camera_move(
    mut camera_q: Query<&mut Transform, With<Camera>>,
    player_q: Query<&GlobalTransform, With<Player>>,
) {
    let mut camera_transform = camera_q.single_mut();
    let player_translation = player_q.single().translation();
    camera_transform.translation = Vec3::new(
        player_translation.x,
        player_translation.y,
        camera_transform.translation.z,
    );
}