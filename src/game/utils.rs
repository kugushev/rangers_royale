use bevy::math::{Vec2, Vec3};

pub trait Vec3Ex {
    fn to_vec2(self) -> Vec2;
}

impl Vec3Ex for Vec3 {
    fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
