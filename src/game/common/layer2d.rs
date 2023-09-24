use bevy::prelude::*;

pub(super) fn build_layer2d(app: &mut App) {
    app.add_systems(PostUpdate, y_sort);
}

#[derive(Component, Copy, Clone)]
pub enum Layer2d {
    Background,
    Ground,
    Character,
    Overlay,
}

impl Layer2d {
    pub fn vec2_to_vec3(&self, vec2: Vec2) -> Vec3 {
        let z = layer_to_z(*self);
        Vec3::new(vec2.x, vec2.y, z)
    }
}

fn y_sort(mut q: Query<(&mut Transform, &Layer2d)>) {
    for (mut transform, layer) in q.iter_mut() {
        let z = layer_to_z(*layer);
        transform.translation.z = z - (1.0 / (1.0 + (2.0f32.powf(-0.01 * transform.translation.y))));
    }
}

fn layer_to_z(layer: Layer2d) -> f32 {
    match layer {
        Layer2d::Background => 0.,
        Layer2d::Ground => 2.,
        Layer2d::Character => 4.,
        Layer2d::Overlay => 6.,
    }
}

