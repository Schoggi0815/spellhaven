use bevy::prelude::*;

use crate::{
    chunk_generation::chunk_generation_plugin::ChunkGenerationPlugin,
    initial_chunk_loader::{
        remove_initial_chunk_loader, spawn_initial_chunk_loader,
    },
    world_generation_state::{
        WorldGenerationState, check_world_done_initializing,
        check_world_gen_started,
    },
};

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<WorldGenerationState>()
            .add_systems(
                Update,
                (
                    check_world_gen_started
                        .run_if(in_state(WorldGenerationState::Waiting)),
                    check_world_done_initializing.run_if(in_state(
                        WorldGenerationState::InitialGeneration,
                    )),
                ),
            )
            .add_systems(
                OnExit(WorldGenerationState::InitialGeneration),
                remove_initial_chunk_loader,
            )
            .add_plugins(ChunkGenerationPlugin)
            .add_observer(spawn_initial_chunk_loader);
    }
}
