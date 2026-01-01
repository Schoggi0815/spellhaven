use bevy::prelude::*;
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

use crate::chunk_generation::{
    block_type::BlockType,
    noise::terrain_noise_group::TerrainNoiseGroup,
    structures::{
        foliage_generation::{
            oak_l_system::OakLSystem, oak_options::OakOptions,
            tree_l_system::LSystem,
        },
        noise_wrapper::NoiseWrapper,
        structure_generator::VoxelStructureMetadata,
        tree_structure_generator::TreeStructureGenerator,
    },
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OakStructureGenerator {
    pub fixed_structure_metadata: VoxelStructureMetadata,
    pub min_thickness_noise: NoiseWrapper,
    pub max_length_noise: NoiseWrapper,
    pub min_length_noise: NoiseWrapper,
    pub max_angle_noise: NoiseWrapper,
    pub start_thickness_noise: NoiseWrapper,
    pub start_x_angle_noise: NoiseWrapper,
    pub start_y_angle_noise: NoiseWrapper,
}

impl TreeStructureGenerator for OakStructureGenerator {
    fn new(
        mut metadata: VoxelStructureMetadata,
        noise_group: &TerrainNoiseGroup,
        world_seed: u64,
    ) -> Self {
        Self::adjust_metadata(&mut metadata);

        Self {
            fixed_structure_metadata: metadata,
            min_thickness_noise: NoiseWrapper::new(
                noise_group.oak_min_thickness.clone(),
                world_seed,
            ),
            max_length_noise: NoiseWrapper::new(
                noise_group.oak_max_length.clone(),
                world_seed,
            ),
            min_length_noise: NoiseWrapper::new(
                noise_group.oak_min_length.clone(),
                world_seed,
            ),
            max_angle_noise: NoiseWrapper::new(
                noise_group.oak_max_angle.clone(),
                world_seed,
            ),
            start_thickness_noise: NoiseWrapper::new(
                noise_group.oak_start_thickness.clone(),
                world_seed,
            ),
            start_x_angle_noise: NoiseWrapper::new(
                noise_group.oak_start_x_angle.clone(),
                world_seed,
            ),
            start_y_angle_noise: NoiseWrapper::new(
                noise_group.oak_start_y_angle.clone(),
                world_seed,
            ),
        }
    }

    fn get_structure_metadata(&self) -> &VoxelStructureMetadata {
        &self.fixed_structure_metadata
    }

    fn grow(
        &self,
        rng: &mut StdRng,
        structure_position: IVec2,
    ) -> Vec<Vec<Vec<BlockType>>> {
        let noise_pos = structure_position.as_dvec2().to_array();

        OakLSystem::grow_new(
            rng,
            &OakOptions {
                min_thickness: self.min_thickness_noise.get(noise_pos) as f32,
                max_length: self.max_length_noise.get(noise_pos) as f32,
                min_length: self.min_length_noise.get(noise_pos) as f32,
                max_angle: self.max_angle_noise.get(noise_pos) as f32,
                start_thickness: self.start_thickness_noise.get(noise_pos)
                    as f32,
                start_x_angle: self.start_x_angle_noise.get(noise_pos) as f32,
                start_y_angle: self.start_y_angle_noise.get(noise_pos) as f32,
            },
            IVec3::from_array(self.fixed_structure_metadata.model_size)
                .as_usizevec3(),
        )
    }
}
