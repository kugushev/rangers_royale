use bevy::log::warn;
use bevy::math::{Vec2, Vec3};

pub trait Vec3Ex {
    fn to_vec2(self) -> Vec2;
}

impl Vec3Ex for Vec3 {
    fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

pub fn find_circle_to_circle_intersections(c0: Vec2, r0: f32, c1: Vec2, r1: f32) -> [Option<Vec2>; 2] {
    let mut result = [None, None];


    let dist = c0.distance(c1);

    if (dist - (r0 + r1)).abs() < f32::EPSILON
    {
        result[0] = Some(c0.lerp(c1, r0 / (r0 + r1)));
        return result;
    }

    // See how many solutions there are.
    if dist > r0 + r1
    {

        warn!("No solutions, the circles are too far apart.");
        return result;
    }

    if dist < (r0 - r1).abs()
    {
        warn!("No solutions, one circle is inside the other.");
        return result;
    }

    if (dist == 0.) && (r0 == r1)
    {
        warn!("No solutions, the circles coincide.");
        return result;
    }

    // Find a and h.
    let a = (r0 * r0 - r1 * r1 + dist * dist) / (2. * dist);
    let h = (r0 * r0 - a * a).sqrt();

    // Find P2.
    let cx2 = c0.x + a * (c1.x - c0.x) / dist;
    let cy2 = c0.y + a * (c1.y - c0.y) / dist;

    // Get the points P3.
    result[0] = Some(Vec2::new(
        cx2 + h * (c1.y - c0.y) / dist,
        cy2 - h * (c1.x - c0.x) / dist,
    ));

    result[1] = Some(Vec2::new(
        cx2 - h * (c1.y - c0.y) / dist,
        cy2 + h * (c1.x - c0.x) / dist,
    ));

    return result;
}

#[derive(Default)]
pub struct AutoResetGate(bool);

impl AutoResetGate {
    pub fn enter(&mut self) -> bool {
        let previous = self.0;
        self.0 = true;
        previous
    }
}