pub(crate) mod animations;

use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Component, SpriteBundle, Transform};
use crate::x_old::components::ui::SkillKey;
use crate::ecs::battle::world::WorldMap;

#[derive(Bundle)]
pub(crate) struct Character {
    pub sprite: SpriteBundle,
    pub orders_handle: CharacterOrdersHandle,
}

impl Character {
    pub fn vec2_to_translation(position: &Vec2) -> Vec3 {
        Vec3::new(position.x, position.y, 2.0)
    }

    pub fn translation_to_vec2(transform: &Transform) -> Vec2 {
        Vec2::new(transform.translation.x, transform.translation.y)
    }
}

#[derive(Component, Default)]
pub(crate) struct CharacterOrdersHandle {
    order: Option<CharacterOrder>,
}

pub(crate) enum CharacterOrder {
    MoveToPosition(Vec2),
    UseSkill(SkillKey, Vec2)
}

impl CharacterOrdersHandle {
    pub fn get_mut(&mut self) -> &mut Option<CharacterOrder> {
        &mut self.order
    }

    pub fn order_move_to_position(&mut self, target: Vec2, map: &WorldMap) {
        let mut x = target.x;
        let mut y = target.y;

        let left = map.get_left();
        let right = map.get_right();
        let top = map.get_top();
        let bottom = map.get_bottom();

        if x < left { x = left; } else if x > right { x = right; }
        if y > top { y = top; } else if y < bottom { y = bottom; }

        self.order = Some(CharacterOrder::MoveToPosition(Vec2::new(x, y)));
    }

    pub fn order_use_skill(&mut self, key: SkillKey, target: Vec2){
        self.order = Some(CharacterOrder::UseSkill(key, target));
    }
}