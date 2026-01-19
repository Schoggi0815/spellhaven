use bevy::prelude::*;
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

use crate::chunk_generation::{
    block_type::BlockType,
    noise::terrain_noise_group::TerrainNoiseGroup,
    structures::{
        foliage_generation::{
            pine_l_system::PineLSystem, pine_options::PineOptions,
            tree_l_system::LSystem,
        },
        noise_wrapper::NoiseWrapper,
        structure_generator::VoxelStructureMetadata,
        tree_structure_generator::TreeStructureGenerator,
    },
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PineStructureGenerator {
    pub fixed_structure_metadata: VoxelStructureMetadata,
    pub stem_piece_length_noise: NoiseWrapper,
    pub stem_thickness_noise: NoiseWrapper,
    pub stem_count_noise: NoiseWrapper,
    pub branch_piece_lenght_noise: NoiseWrapper,
    pub branch_down_angle_noise: NoiseWrapper,
    pub branch_spiral_noise: NoiseWrapper,
    pub branch_droop_noise: NoiseWrapper,
    pub needle_angle_offset_noise: NoiseWrapper,
}

impl TreeStructureGenerator for PineStructureGenerator {
    fn new(
        mut metadata: VoxelStructureMetadata,
        noise_group: &TerrainNoiseGroup,
        world_seed: u64,
    ) -> Self {
        Self::adjust_metadata(&mut metadata);

        Self {
            fixed_structure_metadata: metadata,
            stem_piece_length_noise: NoiseWrapper::new(
                noise_group.pine_stem_piece_length.clone(),
                world_seed,
            ),
            stem_thickness_noise: NoiseWrapper::new(
                noise_group.pine_stem_thickness.clone(),
                world_seed,
            ),
            stem_count_noise: NoiseWrapper::new(
                noise_group.pine_stem_count.clone(),
                world_seed,
            ),
            branch_piece_lenght_noise: NoiseWrapper::new(
                noise_group.pine_branch_piece_lenght.clone(),
                world_seed,
            ),
            branch_down_angle_noise: NoiseWrapper::new(
                noise_group.pine_branch_down_angle.clone(),
                world_seed,
            ),
            branch_spiral_noise: NoiseWrapper::new(
                noise_group.pine_branch_spiral.clone(),
                world_seed,
            ),
            branch_droop_noise: NoiseWrapper::new(
                noise_group.pine_branch_droop.clone(),
                world_seed,
            ),
            needle_angle_offset_noise: NoiseWrapper::new(
                noise_group.pine_needle_angle_offset.clone(),
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

        PineLSystem::grow_new(
            rng,
            &PineOptions {
                stem_piece_length: self
                    .stem_piece_length_noise
                    .get(noise_pos)
                    .value as f32,
                stem_thickness: self.stem_thickness_noise.get(noise_pos).value
                    as f32,
                stem_count: self.stem_count_noise.get(noise_pos).value as f32,
                branch_piece_lenght: self
                    .branch_piece_lenght_noise
                    .get(noise_pos)
                    .value as f32,
                branch_down_angle: self
                    .branch_down_angle_noise
                    .get(noise_pos)
                    .value as f32,
                branch_spiral: self.branch_spiral_noise.get(noise_pos).value
                    as f32,
                branch_droop: self.branch_droop_noise.get(noise_pos).value
                    as f32,
                needle_angle_offset: self
                    .needle_angle_offset_noise
                    .get(noise_pos)
                    .value as f32,
            },
            IVec3::from_array(self.fixed_structure_metadata.model_size)
                .as_usizevec3(),
        )
    }
}
