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

    // app.add_systems(PreUpdate, sync_cursor_position)
    //     .add_systems(OnEnter(GameMode::Battle), setup_cursor_view)
    //     .add_systems(Update, sync_cursor_view.run_if(in_state(GameMode::Battle)));
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

// fn sync_cursor_position(mut cursor: ResMut<MainPlayerCursor>, player_input: Res<PlayerInput>, time: Res<Time>) {
//     if *player_input.horizontal() == 0. && *player_input.vertical() == 0. {
//         return;
//     }
//     const SPEED: f32 = 200.;
//     let mut position = cursor.position;
//     position.x += player_input.horizontal() * SPEED * time.delta_seconds();
//     position.y += player_input.vertical() * SPEED * time.delta_seconds();
//     cursor.position = position;
// }
//
// #[derive(Component)]
// struct CursorView;
//
// const CURSOR_VIEW_LAYER: Layer2d = Layer2d::Overlay;
//
// fn setup_cursor_view(cursor: Res<MainPlayerCursor>,
//                      mut commands: Commands,
//                      asset_server: Res<AssetServer>) {
//     commands.spawn((
//         CursorView,
//         SpriteBundle {
//             texture: asset_server.load("my/Aim.png"),
//             transform: Transform::from_translation(CURSOR_VIEW_LAYER.vec2_to_vec3(cursor.position)),
//             ..default()
//         })
//     );
// }
//
// fn sync_cursor_view(mut cursor_q: Query<&mut Transform, With<CursorView>>, mut cursor: ResMut<MainPlayerCursor>) {
//     let mut cursor_transform = cursor_q.single_mut();
//     cursor_transform.translation = CURSOR_VIEW_LAYER.vec2_to_vec3(cursor.position);
// }

