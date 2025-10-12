use bevy::prelude::*;

use crate::{
    chunk_generation::{
        chunk_start::queue_chunk_tasks,
        chunk_task::{set_generated_chunks, ChunkTaskPool},
        chunk_triangles::ChunkTriangles,
        country::{
            cache_generation_task::{set_generated_caches, CacheTaskPool},
            country_cache::CountryCache,
        },
    },
    chunk_loading::chunk_loader_plugin::ChunkLoaderPlugin,
    world_generation_state::WorldGenerationState,
};

pub struct ChunkGenerationPlugin;

impl Plugin for ChunkGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ChunkLoaderPlugin)
            .init_resource::<ChunkTriangles>()
            .init_resource::<ChunkTaskPool>()
            .init_resource::<CacheTaskPool>()
            .init_resource::<CountryCache>()
            .register_type::<ChunkTriangles>()
            .add_systems(
                Update,
                (
                    queue_chunk_tasks.run_if(
                        in_state(WorldGenerationState::InitialGeneration)
                            .or(in_state(WorldGenerationState::Active)),
                    ),
                    set_generated_chunks,
                    set_generated_caches,
                ),
            );
    }
}
