use bevy::prelude::*;
use crate::game::players::host_cursor::HostCursor;
use crate::game::game_mode::GameMode;

pub(super) fn build_camera(app: &mut App) {
    app.add_systems(Startup, setup);

    // app.add_systems(PreUpdate, handle_camera_move.run_if(in_state(GameMode::Battle)));
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle::default()
    ));
}

// fn handle_camera_move(mut query: Query<&mut Transform, With<MainCamera>>, cursor: Res<HostCursor>) {
//     let p = cursor.position();
//     for mut transform in &mut query {
//         let mut position = transform.translation;
//         position.x = p.x;
//         position.y = p.y;
//         transform.translation = position;
//     }
// }
