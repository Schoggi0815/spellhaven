use bevy::prelude::*;

use crate::{
    chunk_generation::chunk_generation_plugin::ChunkGenerationPlugin,
    world_generation_state::{
        check_world_done_initializing, check_world_gen_started,
        WorldGenerationState,
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
            .add_plugins(ChunkGenerationPlugin);
    }
}
