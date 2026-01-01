use bevy::prelude::*;

use crate::{
    chunk_loading::chunk_loader::ChunkLoader, start_world_gen::StartWorldGen,
};

#[derive(Component)]
pub struct InitialChunkLoader;

pub fn spawn_initial_chunk_loader(
    _: On<StartWorldGen>,
    mut commands: Commands,
) {
    commands.spawn((
        InitialChunkLoader,
        ChunkLoader {
            load_range: 2,
            lod_range: [2, 4, 8, 16, 32, 64, 128, 256],
            ..Default::default()
        },
        Transform::default(),
    ));
}

pub fn remove_initial_chunk_loader(
    mut commands: Commands,
    loaders: Query<Entity, With<InitialChunkLoader>>,
) {
    for entity in loaders {
        commands.entity(entity).despawn();
    }
}
