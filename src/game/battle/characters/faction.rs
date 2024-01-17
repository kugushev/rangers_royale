use bevy::prelude::*;

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub enum Faction {
    Player,
    Enemy,
}

impl Faction {
    pub fn is_rival(&self, other: &Faction) -> bool {
        self != other
    }
}