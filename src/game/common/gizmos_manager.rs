use bevy::prelude::*;
use derive_getters::Getters;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub(super) fn build_gizmos_manager(app: &mut App) {
    app.init_resource::<GizmosManager>();
    app.register_type::<GizmosManager>();
    app.add_plugins(ResourceInspectorPlugin::<GizmosManager>::default());
}

#[derive(Reflect, Resource, Default, Getters, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct GizmosManager {
    show: bool
}