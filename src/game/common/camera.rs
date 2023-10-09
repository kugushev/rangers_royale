use bevy::prelude::*;

pub(super) fn build_camera(app: &mut App) {
    app.add_systems(Startup, setup);
}

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle::default()
    ));
}