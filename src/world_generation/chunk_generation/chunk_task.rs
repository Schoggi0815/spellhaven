use bevy::{
    pbr::ExtendedMaterial,
    prelude::*,
    tasks::{Task, TaskPool, TaskPoolBuilder},
};
use futures_lite::future;

use crate::{
    physics::physics_object::StaticPhysicsObject,
    world_generation::{
        chunk_generation::{
            chunk::Chunk, chunk_generation_result::ChunkGenerationResult,
            chunk_triangles::ChunkTriangles,
        },
        terrain_material::TerrainMaterial,
    },
};

#[derive(Component)]
pub struct ChunkTask(pub Task<ChunkGenerationResult>);

#[derive(Resource)]
pub struct ChunkTaskPool {
    pub task_pool: TaskPool,
}

impl Default for ChunkTaskPool {
    fn default() -> Self {
        Self {
            task_pool: TaskPoolBuilder::new()
                .num_threads(6)
                .stack_size(1_000_000)
                .build(),
        }
    }
}

pub fn set_generated_chunks(
    mut commands: Commands,
    mut chunks: Query<(Entity, &mut ChunkTask)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, TerrainMaterial>>,
    >,
    mut _chunk_triangles: ResMut<ChunkTriangles>,
) {
    for (entity, mut task) in &mut chunks {
        let Some(chunk_generation_result) =
            future::block_on(future::poll_once(&mut task.0))
        else {
            continue;
        };

        let mut current_entity = commands.entity(entity);

        let chunk_pos =
            chunk_generation_result.chunk_pos.get_absolute_chunk_pos(
                chunk_generation_result.chunk_tree_position,
            );

        let chunk_position = chunk_pos.to_absolute(
            chunk_generation_result.chunk_min_height,
            chunk_generation_result.chunk_pos.lod,
        );

        current_entity.remove::<ChunkTask>().insert((
            Chunk {
                tree_position: chunk_generation_result.chunk_tree_position,
                chunk_height: chunk_generation_result.chunk_stack_offset,
                generate_above: chunk_generation_result.generate_above,
                lod_position: chunk_generation_result.chunk_pos,
            },
            Transform::from_translation(chunk_position),
        ));

        if let Some(collider) = chunk_generation_result.mesh_result.collider {
            current_entity.insert((collider, StaticPhysicsObject));
        }

        // let triangle_count = mesh.indices().unwrap().len() / 3;
        // let result_lod = chunk_generation_result.chunk_pos.lod.usize();
        // chunk_triangles.0[result_lod - 1] += triangle_count as u64;

        let opaque_mesh = chunk_generation_result.mesh_result.opaque_mesh;
        let transparent_mesh =
            chunk_generation_result.mesh_result.transparent_mesh;

        let chunk_shader_pos = chunk_position;

        let terrain_material = TerrainMaterial {
            chunk_position: chunk_shader_pos,
            lod_multiplier: chunk_generation_result
                .chunk_pos
                .lod
                .multiplier_i32() as u32,
        };

        current_entity.with_children(|child_spawner| {
            if let Some(mesh) = opaque_mesh {
                child_spawner.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.add(ExtendedMaterial {
                        base: StandardMaterial {
                            perceptual_roughness: 1.,
                            ..Default::default()
                        },
                        extension: terrain_material.clone(),
                    })),
                ));
            }

            if let Some(mesh) = transparent_mesh {
                child_spawner.spawn((
                    Mesh3d(meshes.add(mesh)),
                    MeshMaterial3d(materials.add(ExtendedMaterial {
                        base: StandardMaterial {
                            alpha_mode: AlphaMode::AlphaToCoverage,
                            perceptual_roughness: 1.,
                            ..Default::default()
                        },
                        extension: terrain_material,
                    })),
                ));
            }
        });
    }
}
