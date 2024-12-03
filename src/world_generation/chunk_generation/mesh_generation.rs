use std::ops::Neg;

use crate::world_generation::chunk_generation::{CHUNK_SIZE, VOXEL_SIZE};
use crate::world_generation::voxel_world::ChunkLod;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

use super::voxel_types::VoxelData;

pub fn generate_mesh(
    blocks: &VoxelData,
    min_height: i32,
    chunk_lod: ChunkLod,
) -> Option<(Mesh, Vec<Vec3>, Vec<[u32; 3]>)> {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut triangles: Vec<[u32; 3]> = Vec::new();
    // let mut colors: Vec<[f32; 4]> = Vec::new();

    fn rotate_into_direction<T: Vec3Swizzles + Neg<Output = T>>(vector: T, direction: IVec3) -> T {
        match direction {
            IVec3::X | IVec3::NEG_X => vector.xzy(),
            IVec3::Y | IVec3::NEG_Y => vector.yxz(),
            IVec3::Z | IVec3::NEG_Z => vector.zyx(),
            _ => vector,
        }
    }

    let mut generate_sides = |direction: IVec3| {
        for i in 1..CHUNK_SIZE + 1 {
            let mut done_faces = [[false; CHUNK_SIZE]; CHUNK_SIZE];
            for j in 1..CHUNK_SIZE + 1 {
                for k in 1..CHUNK_SIZE + 1 {
                    let current_pos =
                        rotate_into_direction(IVec3::new(i as i32, j as i32, k as i32), direction);

                    let height_dir = rotate_into_direction(IVec3::Y, direction);
                    let width_dir = rotate_into_direction(IVec3::Z, direction);

                    let width_pos = (current_pos * width_dir).max_element();
                    let height_pos = (current_pos * height_dir).max_element();

                    if done_faces[width_pos as usize - 1][height_pos as usize - 1]
                        || blocks.is_air(current_pos)
                        || !blocks.is_air(current_pos + direction)
                    {
                        continue;
                    }

                    let mut height = 1;
                    let mut width = 1;

                    while height_pos + height <= CHUNK_SIZE as i32
                        && !done_faces[width_pos as usize - 1]
                            [height_pos as usize + height as usize - 1]
                        && !blocks.is_air(current_pos + (height_dir * height))
                        && blocks.is_air(current_pos + (height_dir * height) + direction)
                    {
                        height += 1;
                    }

                    while width_pos + width <= CHUNK_SIZE as i32
                        && (0..height).all(|height| {
                            !done_faces[width_pos as usize + width as usize - 1]
                                [height_pos as usize + height as usize - 1]
                                && !blocks.is_air(
                                    current_pos
                                        + (width_dir * width as i32)
                                        + (height_dir * height as i32),
                                )
                                && blocks.is_air(
                                    current_pos
                                        + (width_dir * width as i32)
                                        + (height_dir * height as i32)
                                        + direction,
                                )
                        })
                    {
                        width += 1;
                    }

                    for x in width_pos..width_pos + width {
                        for y in height_pos..height_pos + height {
                            done_faces[x as usize - 1][y as usize - 1] = true;
                        }
                    }

                    let height = height as f32 - 1.;
                    let width = width as f32 - 1.;

                    let positions_count = positions.len() as u32;

                    let vertex_pos = current_pos.as_vec3();

                    let direction_adder = direction * (direction.min_element().abs());

                    positions.extend_from_slice(&[
                        (vertex_pos
                            + (rotate_into_direction(Vec3::new(0.5, -0.5, -0.5), direction))
                            + direction_adder.as_vec3())
                        .to_array(),
                        (vertex_pos
                            + (rotate_into_direction(
                                Vec3::new(0.5, -0.5, 0.5 + width),
                                direction,
                            ))
                            + direction_adder.as_vec3())
                        .to_array(),
                        (vertex_pos
                            + (rotate_into_direction(
                                Vec3::new(0.5, 0.5 + height, 0.5 + width),
                                direction,
                            ))
                            + direction_adder.as_vec3())
                        .to_array(),
                        (vertex_pos
                            + (rotate_into_direction(
                                Vec3::new(0.5, 0.5 + height, -0.5),
                                direction,
                            ))
                            + direction_adder.as_vec3())
                        .to_array(),
                    ]);

                    normals.extend_from_slice(&[
                        direction.as_vec3().to_array(),
                        direction.as_vec3().to_array(),
                        direction.as_vec3().to_array(),
                        direction.as_vec3().to_array(),
                    ]);

                    let invert = !direction.min_element() < 0;

                    triangles.extend_from_slice(&[
                        [
                            positions_count + 0,
                            positions_count + if invert { 1 } else { 3 },
                            positions_count + if invert { 3 } else { 1 },
                        ],
                        [
                            positions_count + 1,
                            positions_count + if invert { 2 } else { 3 },
                            positions_count + if invert { 3 } else { 2 },
                        ],
                    ]);
                }
            }
        }
    };

    generate_sides(IVec3::X);
    generate_sides(IVec3::NEG_X);
    generate_sides(IVec3::Z);
    generate_sides(IVec3::NEG_Z);
    generate_sides(IVec3::Y);
    generate_sides(IVec3::NEG_Y);

    if triangles.is_empty() {
        return None;
    }

    for position in positions.iter_mut() {
        position[0] = (position[0] - 0.5) * VOXEL_SIZE * chunk_lod.multiplier_f32() + 0.5;
        position[1] =
            (position[1] + min_height as f32 - 0.5) * VOXEL_SIZE * chunk_lod.multiplier_f32() + 0.5;
        position[2] = (position[2] - 0.5) * VOXEL_SIZE * chunk_lod.multiplier_f32() + 0.5;
    }

    let mut mesh_triangles: Vec<u32> = Vec::new();

    for triangle in &triangles {
        mesh_triangles.push(triangle[0]);
        mesh_triangles.push(triangle[1]);
        mesh_triangles.push(triangle[2]);
    }

    let collider_positions = if chunk_lod == ChunkLod::Full {
        positions
            .clone()
            .iter()
            .map(|position| Vec3::new(position[0], position[1], position[2]))
            .collect()
    } else {
        Vec::new()
    };

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    mesh.insert_indices(Indices::U32(mesh_triangles));

    Some((
        mesh,
        collider_positions,
        if chunk_lod == ChunkLod::Full {
            triangles
        } else {
            Vec::new()
        },
    ))
}
