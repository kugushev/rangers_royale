use bevy::prelude::*;
use crate::x_old::components::deck::{SkillsHand, Deck, HandSlot};
use crate::x_old::components::Player;
use crate::x_old::components::ui::{SkillBlock, SkillDeck, SkillKey, SkillsRowBlock};

pub(crate) fn build_ui_systems(app: &mut App) {
    app.add_startup_system(setup_ui)
        .add_system(sync_skills_ui)
        .add_system(sync_deck_ui);
}


fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        SkillsRowBlock
    )).with_children(|parent| {
        add_skill_block(&asset_server, parent, SkillBlock(SkillKey::Q));
        add_skill_block(&asset_server, parent, SkillBlock(SkillKey::W));
        add_skill_block(&asset_server, parent, SkillBlock(SkillKey::E));
        add_skill_block(&asset_server, parent, SkillBlock(SkillKey::R));
        add_deck_block(&asset_server, parent);
    });
}

fn add_skill_block(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder, block: SkillBlock) {
    let key_text = block.get_text();
    parent.spawn(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(30.0)),
                margin: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: Color::rgb(0.3, 0.3, 1.0).into(),
            ..default()
        }
    ).with_children(|parent| {
        parent.spawn(TextBundle::from_section(key_text, TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            ..default()
        }));
        parent.spawn((
            TextBundle::from_section("<Not Specified>", TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                ..default()
            }).with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    size: Size::all(Val::Percent(100.0)),
                    position: UiRect::new(Val::Px(10.0), Val::Px(10.0), Val::Px(5.0), Val::Px(5.0)),
                    ..default()
                }),
            block
        ));
    });
}

fn add_deck_block(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(45.0), Val::Px(30.0)),
            margin: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        background_color: Color::rgb(1.0, 0.3, 0.1).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(":(", TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                ..default()
            }).with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    size: Size::all(Val::Percent(100.0)),
                    position: UiRect::new(Val::Px(10.0), Val::Px(10.0), Val::Px(5.0), Val::Px(5.0)),
                    ..default()
                }),
            SkillDeck
        ));
    });
}

fn sync_skills_ui(player: Query<&SkillsHand, With<Player>>, mut skills_ui: Query<(&SkillBlock, &mut Text)>) {
    let hand = player.single();

    for (block, mut text_ui) in &mut skills_ui {
        let slot = match block.0 {
            SkillKey::Q => { &hand.q }
            SkillKey::W => { &hand.w }
            SkillKey::E => { &hand.e }
            SkillKey::R => { &hand.r }
        };

        text_ui.sections[0].value = match slot {
            HandSlot::Obtained(card) => { card.get_text().to_string() }
            HandSlot::Cooldown(cooldown) => { cooldown.to_string() }
            HandSlot::Empty => { "---".to_string() }
        };
    }
}

fn sync_deck_ui(player: Query<&Deck, With<Player>>, mut deck_ui: Query<&mut Text, With<SkillDeck>>) {
    let deck = player.single();
    let mut text = deck_ui.single_mut();
    text.sections[0].value = deck.0.len().to_string();
}