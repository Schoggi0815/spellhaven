use bevy::prelude::*;
use rand::{Rng, rngs::StdRng};
use serde::{Deserialize, Serialize};

use crate::chunk_generation::{
    block_type::BlockType,
    noise::terrain_noise::TerrainNoise,
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

#[derive(Clone, Serialize, Deserialize)]
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
    fn new(mut metadata: VoxelStructureMetadata, rng: &mut StdRng) -> Self {
        Self::adjust_metadata(&mut metadata);

        Self {
            fixed_structure_metadata: metadata,
            min_thickness_noise: NoiseWrapper::new(
                TerrainNoise::new_basic_simplex(0.5_f64.powi(5), 0.5, 0.7),
                rng.random(),
            ),
            max_length_noise: NoiseWrapper::new(
                TerrainNoise::new_basic_simplex(0.5_f64.powi(5), 6.5, 7.5),
                rng.random(),
            ),
            min_length_noise: NoiseWrapper::new(
                TerrainNoise::new_basic_simplex(0.5_f64.powi(5), 3.5, 4.5),
                rng.random(),
            ),
            max_angle_noise: NoiseWrapper::new(
                TerrainNoise::new_basic_simplex(0.5_f64.powi(5), 45.0, 65.0),
                rng.random(),
            ),
            start_thickness_noise: NoiseWrapper::new(
                TerrainNoise::new_basic_simplex(0.5_f64.powi(5), 1.8, 3.0),
                rng.random(),
            ),
            start_x_angle_noise: NoiseWrapper::new(
                TerrainNoise::new_basic_simplex(0.5_f64.powi(5), -10.0, 10.0),
                rng.random(),
            ),
            start_y_angle_noise: NoiseWrapper::new(
                TerrainNoise::new_basic_simplex(0.5_f64.powi(5), -10.0, 10.0),
                rng.random(),
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
