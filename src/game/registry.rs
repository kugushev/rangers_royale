use serde::Deserialize;

pub const CHARACTER_RADIUS: f32 = 30.;

#[derive(Copy, Clone, Debug)]
pub enum AttackRange {
    Hand,
    Regular,
    Pole,
}

impl AttackRange {
    pub fn distance(&self) -> f32 {
        match self {
            AttackRange::Hand => CHARACTER_RADIUS * 2. * 1.1,
            AttackRange::Regular => CHARACTER_RADIUS * 2. * 1.5,
            AttackRange::Pole => CHARACTER_RADIUS * 2. * 2.
        }
    }
}