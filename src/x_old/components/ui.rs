use bevy::prelude::*;


#[derive(Component)]
pub struct SkillsRowBlock;

#[derive(Component)]
pub struct SkillBlock(pub SkillKey);

pub enum SkillKey {
    Q,
    W,
    E,
    R,
}

impl SkillBlock {
    pub fn get_text(&self) -> &'static str {
        match self.0 {
            SkillKey::Q => { "Q" }
            SkillKey::W => { "W" }
            SkillKey::E => { "E" }
            SkillKey::R => { "R" }
        }
    }
}

#[derive(Component)]
pub struct SkillDeck;