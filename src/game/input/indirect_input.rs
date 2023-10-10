use bevy::prelude::*;
use bevy::math::Vec2;
use bevy::window::PrimaryWindow;
use derive_getters::Getters;

pub(super) fn build_indirect_input(app: &mut App) {
    app.insert_resource(IndirectInputCursor::default());

    app.add_systems(First, handle_mouse_position)
        .add_systems(First, handle_mouse_input);
}

#[derive(Resource, Getters, Default)]
pub struct IndirectInputCursor {
    position: Vec2,
    do_action: Option<()>,
    do_select: Option<()>,
    pub on_collider: bool,
}

fn handle_mouse_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor: ResMut<IndirectInputCursor>,
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

fn handle_mouse_input(mouse_button: Res<Input<MouseButton>>, mut cursor: ResMut<IndirectInputCursor>) {
    let for_button = |button|
        if mouse_button.just_pressed(button) { Some(()) } else { None };

    cursor.do_action = for_button(MouseButton::Right);
    cursor.do_select = for_button(MouseButton::Left);
}