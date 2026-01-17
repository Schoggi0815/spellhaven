use std::f32::consts::PI;

use bevy::prelude::*;
use noise::NoiseFn;

use crate::{
    chunk_generation::{
        VOXEL_SIZE,
        chunk_lod::{ChunkLod, MAX_LOD},
    },
    chunk_loading::{chunk_tree::ChunkTreePos, lod_position::LodPosition},
};

#[derive(Component)]
pub struct VisualChunkLoader;

impl VisualChunkLoader {
    pub fn get_min_lod(
        &self,
        terrain_noise: &Box<dyn NoiseFn<f64, 2> + Send + Sync>,
        lod_pos: LodPosition,
        tree_pos: ChunkTreePos,
        projection: &Projection,
        camera_transform: &GlobalTransform,
        camera: &Camera,
    ) -> ChunkLod {
        let Projection::Perspective(perspective) = projection else {
            return MAX_LOD;
        };

        let vertical_fov = perspective.fov / PI * 2.;

        let center = lod_pos.get_center(tree_pos);

        let corner_height = terrain_noise
            .get((center / VOXEL_SIZE).as_dvec2().to_array())
            as f32
            * VOXEL_SIZE;

        let corner_pos = Vec3::new(center.x, corner_height, center.y);

        let ndc = camera.world_to_ndc(camera_transform, corner_pos);

        let Some(ndc) = ndc else {
            return MAX_LOD;
        };

        let leeway = lod_pos.lod.multiplier_f32() / 256.;

        if ndc.x.abs() > 1. + leeway || ndc.y.abs() > 1. + leeway || ndc.z < 0.
        {
            return MAX_LOD;
        }

        let ratio = ndc.z * 1_000. / vertical_fov;

        ChunkLod::from_fraction(ratio).min(MAX_LOD)
    }
}
