use bevy::prelude::*;
use crate::game::battle::characters::Character;
use crate::game::battle::characters::character_state::CharacterState;
use crate::game::battle::characters::attacker::Attacker;
use crate::game::battle::value_objects::Damage;
use crate::game::registry::CharacterOrigin;

pub(super) fn build_hit_points(app: &mut App) {
    app.add_systems(First, setup_hp)
        .add_systems(Last, update_hp_text);
}

#[derive(Component, Default)]
pub struct HitPoints {
    current: f32,
    max: f32,
}

#[derive(Component)]
pub struct HitPointsMark;

#[derive(Bundle)]
pub struct HitPointsMarkBundle {
    marker: HitPointsMark,
    text: Text2dBundle,
}

impl HitPointsMarkBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let text_style = TextStyle {
            font: font.clone(),
            font_size: 30.0,
            color: Color::WHITE,
        };
        Self {
            marker: HitPointsMark,
            text: Text2dBundle {
                text: Text::from_section("42", text_style),
                ..default()
            },
        }
    }
}

impl HitPoints {
    pub fn suffer(&mut self, damage: Damage) {
        self.current -= damage.amount();
        // println!("Suffer: {}/{}", self.current, self.max);
    }

    pub fn is_dead(&self) -> bool {
        if self.max == f32::default() {
            return false;
        }

        self.current <= 0.0
    }
}

fn setup_hp(mut query: Query<(&mut HitPoints, &Character)>) {
    for (mut hit_points, character) in &mut query {
        let max = match character.origin() {
            CharacterOrigin::Red => 50.0,
            CharacterOrigin::Candy => 50.0,
            CharacterOrigin::Knife => 50.0,
            CharacterOrigin::Rose => 50.0,
            CharacterOrigin::Orc => 40.0
        };

        if hit_points.max != max {
            hit_points.max = max;
            hit_points.current = max;
        }
    }
}

fn update_hp_text(mut query: Query<(&HitPoints, &mut Children)>, mut mark_q: Query<&mut Text, With<HitPointsMark>>) {
    for (hp, children) in &mut query {
        for child in &children {
            if let Ok(mut text) = mark_q.get_mut(*child) {
                text.sections[0].value = hp.current.to_string()
            }
        }
    }
}