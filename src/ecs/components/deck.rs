use bevy::prelude::*;

#[derive(Component)]
pub struct Deck(pub Vec<Card>);

impl Deck {
    pub fn take(&mut self) -> Option<Card> {
        self.0.pop()
    }
}

pub enum Card {
    MagicMissile
}

impl Card {
    pub fn get_text(&self) -> &'static str {
        match self {
            Card::MagicMissile => { "Magic Missile" }
        }
    }
}

#[derive(Component, Default)]
pub struct CharacterHand {
    pub q: HandSlot,
    pub w: HandSlot,
    pub e: HandSlot,
    pub r: HandSlot,
}

pub enum HandSlot {
    Obtained(Card),
    Cooldown(f32),
    Empty,
}

impl Default for HandSlot {
    fn default() -> Self {
        HandSlot::Empty
    }
}