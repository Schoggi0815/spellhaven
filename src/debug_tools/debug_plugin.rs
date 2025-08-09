use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::{
    debug_tools::{
        chunk_gizmos::{draw_path_gizmos, setup_gizmo_settings},
        debug_resource::SpellhavenDebugResource,
        physics_debug::{PhysicsDebugResource, render_physics_debug},
    },
    world_generation::generation_options::GenerationOptionsResource,
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
                    draw_path_gizmos
                        .run_if(resource_exists::<GenerationOptionsResource>),
                    render_physics_debug
                        .run_if(resource_exists::<PhysicsDebugResource>),
                ),
            );
    }
}
