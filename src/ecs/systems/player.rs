use std::ops::Deref;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::ecs::components::{Character, CharacterOrdersHandle, Player};
use crate::ecs::components::deck::{CharacterHand, Deck};
use crate::ecs::components::deck::Card::MagicMissile;
use crate::ecs::resources::WorldMap;

pub(crate) fn build_player_systems(app: &mut App) {
    app.add_startup_system(setup_player)
        .add_system(handle_mouse_input)
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
        CharacterHand::default(),
        Player
    ));
}

fn handle_mouse_input(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    player_q: Query<&mut CharacterOrdersHandle, With<Player>>,
    world_map: Res<WorldMap>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    inner(windows, camera_q, player_q, world_map.deref());

    fn inner(windows: Query<&Window, With<PrimaryWindow>>,
             camera_q: Query<(&GlobalTransform, &Camera)>,
             mut player_q: Query<&mut CharacterOrdersHandle, With<Player>>,
             world_map: &WorldMap) -> Option<()> {
        let cursor_position = windows.single().cursor_position()?;
        let (trans, cam) = camera_q.single();
        let world_position = cam.viewport_to_world_2d(trans, cursor_position)?;

        let mut orders_handle = player_q.single_mut();
        orders_handle.order_move_to_position(world_position, world_map);
        Some(())
    }
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