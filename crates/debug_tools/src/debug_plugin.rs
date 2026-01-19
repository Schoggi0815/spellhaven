use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use debug_resource::debug_resource::SpellhavenDebugResource;

use crate::{
    chunk_gizmos::{draw_path_gizmos, setup_gizmo_settings},
    physics_debug::{PhysicsDebugResource, render_physics_debug},
    terrain_derivative::draw_terrain_derivative,
};

pub struct SpellhavenDebugPlugin;

impl Plugin for SpellhavenDebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpellhavenDebugResource>()
            .register_type::<SpellhavenDebugResource>()
            .add_plugins(
                ResourceInspectorPlugin::<SpellhavenDebugResource>::default(),
            )
            .add_systems(Startup, setup_gizmo_settings)
            .add_systems(
                Update,
                (
                    draw_path_gizmos,
                    draw_terrain_derivative,
                    render_physics_debug
                        .run_if(resource_exists::<PhysicsDebugResource>),
                ),
            );
    }
}
