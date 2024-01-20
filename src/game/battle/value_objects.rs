use bevy::prelude::Entity;

#[derive(Copy, Clone)]
pub struct Damage(f32, Entity);

impl Damage {
    pub fn new(amount: f32, source: Entity) -> Self {
        Self(amount, source)
    }
    pub fn amount(&self) -> f32 { self.0 }
    pub fn source(&self) -> Entity { self.1 }
}