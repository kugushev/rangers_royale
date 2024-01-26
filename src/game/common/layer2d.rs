use bevy::prelude::*;
use crate::game::battle::encounter::encounter_map::EncounterMap;

pub(super) fn build_layer2d(app: &mut App) {
    app.add_systems(PostUpdate, y_sort);
}

pub const LAYER_SIZE: f32 = 2.;

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

fn y_sort(mut query: Query<(&mut Transform, &Layer2d)>, world_map: Res<EncounterMap>) {
    for (mut transform, layer) in &mut query {
        let base = layer_to_z(*layer);
        let shift = transform.translation.y / world_map.get_height();
        transform.translation.z = base - shift.clamp(-1., 1.);
    }
}

fn layer_to_z(layer: Layer2d) -> f32 {
    match layer {
        Layer2d::Background => 0.,
        Layer2d::Ground => LAYER_SIZE,
        Layer2d::Character => 2. * LAYER_SIZE,
        Layer2d::Overlay => 3. * LAYER_SIZE,
    }
}

