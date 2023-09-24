use bevy::prelude::*;
use bevy::math::Vec2;
use derive_getters::Getters;

use bevy::sprite::MaterialMesh2dBundle;
use crate::game::common::camera::MainCamera;
use crate::game::common::layer2d::Layer2d;
use crate::game::common::player_input::PlayerInput;
use crate::game::utils::Vec3Ex;

pub(super) fn build_cursor(app: &mut App) {
    app.insert_resource(GameCursor {
        position: Vec2::ZERO
    });

    app.add_systems(PreUpdate, sync_cursor_position)
        .add_systems(Startup, setup_cursor_view)
        .add_systems(Update, sync_cursor_view);
}

#[derive(Resource, Getters)]
pub struct GameCursor {
    position: Vec2,
}

fn sync_cursor_position(mut cursor: ResMut<GameCursor>, player_input: Res<PlayerInput>, time: Res<Time>) {
    if *player_input.horizontal() == 0. && *player_input.vertical() == 0. {
        return;
    }
    const SPEED: f32 = 200.;
    let mut position = cursor.position;
    position.x += player_input.horizontal() * SPEED * time.delta_seconds();
    position.y += player_input.vertical() * SPEED * time.delta_seconds();
    cursor.position = position;
}

#[derive(Component)]
struct CursorView;

const CURSOR_VIEW_LAYER: Layer2d = Layer2d::Overlay;

fn setup_cursor_view(cursor: Res<GameCursor>,
                     mut commands: Commands,
                     asset_server: Res<AssetServer>) {
    commands.spawn((
        CursorView,
        SpriteBundle {
            texture: asset_server.load("my/Aim.png"),
            transform: Transform::from_translation(CURSOR_VIEW_LAYER.vec2_to_vec3(cursor.position)),
            ..default()
        })
    );
}

fn sync_cursor_view(mut cursor_q: Query<&mut Transform, With<CursorView>>, mut cursor: ResMut<GameCursor>) {
    let mut cursor_transform = cursor_q.single_mut();
    cursor_transform.translation = CURSOR_VIEW_LAYER.vec2_to_vec3(cursor.position);
}

