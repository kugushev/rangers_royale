use bevy::prelude::*;
use bevy::math::Vec2;
use bevy::window::PrimaryWindow;
use derive_getters::Getters;

pub(super) fn build_lead_cursor(app: &mut App) {
    app.insert_resource(HostCursor {
        position: Vec2::ZERO,
        action_command: None,
        select_command: None,
    });

    app.add_systems(First, handle_mouse_position)
        .add_systems(First, handle_mouse_input);
}

#[derive(Resource, Getters)]
pub struct HostCursor {
    position: Vec2,
    action_command: Option<()>,
    select_command: Option<()>,
}

fn handle_mouse_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor: ResMut<HostCursor>,
) {
    let cursor_position = match windows.single().cursor_position() {
        None => { return; }
        Some(p) => p,
    };
    let (trans, cam) = camera_q.single();
    let world_position = cam.viewport_to_world_2d(trans, cursor_position);
    if let Some(p) = world_position {
        cursor.position = p;
    }
}

fn handle_mouse_input(mouse_button: Res<Input<MouseButton>>, mut cursor: ResMut<HostCursor>) {
    let for_button = |button|
        if mouse_button.just_pressed(button) { Some(()) } else { None };

    cursor.action_command = for_button(MouseButton::Right);
    cursor.select_command = for_button(MouseButton::Left);
}