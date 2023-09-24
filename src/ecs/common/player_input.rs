use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use derive_getters::Getters;

pub(super) fn build_player_input(app: &mut App) {
    app.insert_resource(CursorPosition(Vec2::ZERO))
        .insert_resource(PlayerInput::default());
    app.add_systems(First, handle_mouse_position)
        .add_systems(PreUpdate, handle_mouse_input);
}

#[derive(Resource, Default, Getters)]
pub struct PlayerInput {
    action_command: Option<Vec2>,
    select_command: Option<Vec2>,
}

#[derive(Resource, Copy, Clone)]
pub struct CursorPosition(pub Vec2);

fn handle_mouse_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor: ResMut<CursorPosition>,
) {
    let cursor_position = match windows.single().cursor_position() {
        None => { return; }
        Some(p) => p,
    };
    let (trans, cam) = camera_q.single();
    let world_position = cam.viewport_to_world_2d(trans, cursor_position);
    if let Some(p) = world_position {
        cursor.0 = p;
    }
}

fn handle_mouse_input(mouse_button: Res<Input<MouseButton>>, cursor: Res<CursorPosition>, mut player_input: ResMut<PlayerInput>) {
    let for_button = |button| {
        if mouse_button.just_pressed(button) {
            Some(cursor.0)
        } else {
            None
        }
    };

    player_input.action_command = for_button(MouseButton::Right);
    player_input.select_command = for_button(MouseButton::Left);
}

