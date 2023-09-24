use bevy::prelude::*;
use crate::game::common::game_cursor::GameCursor;
use crate::game::common::player_input::PlayerInput;
use crate::game::scenes::GameScene;

pub(super) fn build_camera(app: &mut App) {
    app.add_systems(Startup, setup);

    app.add_systems(PreUpdate, handle_camera_move.run_if(in_state(GameScene::Battle)));
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle::default()
    ));
}

fn handle_camera_move(mut query: Query<&mut Transform, With<MainCamera>>, cursor: Res<GameCursor>) {
    let p = cursor.position();
    for mut transform in &mut query {
        let mut position = transform.translation;
        position.x = p.x;
        position.y = p.y;
        transform.translation = position;
    }
}
