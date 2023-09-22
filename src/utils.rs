use bevy::prelude::*;

const Z_SHIFT: f32 = 2.0;

pub trait Vec2toVec3 {
    fn to_vec3(self) -> Vec3;
}

impl Vec2toVec3 for Vec2 {
    fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, Z_SHIFT)
    }
}

trait Vec3toVec2 {
    fn to_vec2(self) -> Vec2;
}

impl Vec3toVec2 for Vec3 {
    fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}