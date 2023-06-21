use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::ecs::components::{Character, CharacterOrdersHandle, Player};
use crate::ecs::components::CharacterOrder::MoveToPosition;

pub(crate) fn build_player_systems(app: &mut App) {
    app.add_startup_system(setup_player)
        .add_system(handle_mouse_input);
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = SpriteBundle {
        texture: asset_server.load("paid/player.png"),
        transform: Transform::from_translation(Character::vec2_to_translation(&Vec2::new(0.0, 0.0))),
        ..default()
    };

    commands.spawn((Character {
        sprite,
        orders_handle: default(),
    }, Player));
}

fn handle_mouse_input(
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    player_q: Query<(&mut CharacterOrdersHandle, &Player)>,
) {

    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    inner(windows, camera_q, player_q);

    fn inner(windows: Query<&Window, With<PrimaryWindow>>,
             camera_q: Query<(&GlobalTransform, &Camera)>,
             mut player_q: Query<(&mut CharacterOrdersHandle, &Player)>) -> Option<()> {
        let cursor_position = windows.single().cursor_position()?;
        let (trans, cam) = camera_q.single();
        let world_position = cam.viewport_to_world_2d(trans, cursor_position)?;

        let (mut orders_handle, _) = player_q.single_mut();
        orders_handle.order = Some(MoveToPosition(world_position));
        Some(())
    }
}