use bevy::prelude::*;
use crate::game::game_mode::GameMode;

pub(super) fn build_champion(app: &mut App) {
    app.add_systems(OnEnter(GameMode::Chamption), show_ui);
}

fn show_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }).with_children(|b1| {
        b1.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }).with_children(|b2| {
            b2.spawn(TextBundle::from_section(
                "You Win! :)",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 100.0,
                    ..default()
                },
            ).with_text_alignment(TextAlignment::Center));
        });
    });
}