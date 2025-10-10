use crate::chunk_generation::{
    block_type::BlockType,
    noise::{
        terrain_noise::{TERRAIN_NOISE_FILE_PATH, TerrainNoise},
        terrain_noise_type::{ConstantValue, TerrainNoiseType},
    },
    structures::{
        oak_structure_generator::OakStructureGenerator,
        structure_generator::VoxelStructureMetadata,
        structure_generators::StructureGenerators,
        tree_structure_generator::TreeStructureGenerator,
    },
};
use bevy::prelude::*;
use noise::NoiseFn;
use rand::SeedableRng;
use rand::prelude::StdRng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utils::file_utils::read_ron_from_file;

fn get_seeded_white_noise() -> TerrainNoise {
    TerrainNoise::new(
        0,
        vec![TerrainNoiseType::ConstantValue {
            value: ConstantValue::F64(1.0),
        }],
    )
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerationOptions {
    pub seed: u64,
    pub structure_generators: Vec<Arc<Box<StructureGenerators>>>,
    pub structure_assets: Vec<StructureAsset>,
    pub generate_paths: bool,
    pub terrain_noise: TerrainNoise,
}

impl GenerationOptions {
    pub fn from_seed(seed: u64) -> Self {
        // let tree_house = vox_data_to_structure_data(
        //     &from_file("assets/tree_house.vox").unwrap(),
        // );
        // let box_structure =
        //     vox_data_to_structure_data(&from_file("assets/box.vox").unwrap());
        let terrain_noise: TerrainNoise =
            read_ron_from_file(TERRAIN_NOISE_FILE_PATH)
                .expect("Failed loading terrain noise config.");

        // let tree_model: StructureModel =
        //     read_ron_from_file("assets/tree_test.ron")
        //         .expect("Failed to load tree model.");

        Self {
            seed,
            terrain_noise,
            generate_paths: false,
            structure_generators: vec![
                Arc::new(Box::new(StructureGenerators::Oak(
                    OakStructureGenerator::new(VoxelStructureMetadata::new(
                        [27, 27, 27],
                        [64, 64],
                        [24, 16],
                        get_seeded_white_noise(),
                    )),
                ))),
                Arc::new(Box::new(StructureGenerators::Oak(
                    OakStructureGenerator::new(VoxelStructureMetadata::new(
                        [27, 27, 27],
                        [64, 64],
                        [43, 52],
                        get_seeded_white_noise(),
                    )),
                ))),
                Arc::new(Box::new(StructureGenerators::Oak(
                    OakStructureGenerator::new(VoxelStructureMetadata::new(
                        [27, 27, 27],
                        [64, 64],
                        [10, 4],
                        get_seeded_white_noise(),
                    )),
                ))),
                // Arc::new(Box::new(FixedStructureGenerator {
                //     fixed_structure_model: Arc::new(tree_model.blocks),
                //     fixed_structure_metadata: VoxelStructureMetadata {
                //         model_size: tree_model.model_size.to_array(),
                //         generation_size: [10, 10],
                //         grid_offset: [7, 4],
                //         generate_debug_blocks: false,
                //         debug_rgb_multiplier: [1., 1., 1.],
                //         noise: get_seeded_white_noise(rng.random()),
                //     },
                // })),
            ],
            structure_assets: vec![
                // StructureAsset {
                //     _blocks: (*box_structure.0).clone(),
                // }
            ],
        }
    }

    pub fn get_terrain_noise(&self) -> impl NoiseFn<f64, 2> {
        self.terrain_noise
            .get_noise_fn(&mut StdRng::seed_from_u64(self.seed + 1))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructureAsset {
    pub _blocks: Vec<Vec<Vec<BlockType>>>,
}

// fn vox_data_to_blocks(vox_data: &VoxData) -> Vec<Vec<Vec<BlockType>>> {
//     let model = vox_data.models.first().unwrap();
//     let mut result: Vec<Vec<Vec<BlockType>>> =
//         Vec::with_capacity(model.size.x as usize);
//     for x in 0..model.size.x {
//         result.push(Vec::with_capacity(model.size.z as usize));
//         for y in 0..model.size.z {
//             result[x as usize].push(Vec::with_capacity(model.size.y as usize));
//             for _ in 0..model.size.y {
//                 result[x as usize][y as usize].push(BlockType::Air);
//             }
//         }
//     }

//     for voxel in model.voxels.iter() {
//         //let color = vox_data.palette.colors[voxel.color_index.0 as usize];
//         let [x, y, z] = [
//             voxel.point.x as usize,
//             voxel.point.y as usize,
//             voxel.point.z as usize,
//         ];
//         result[x][z][y] = BlockType::Stone;
//     }

//     result
// }

// fn vox_data_model_size(vox_data: &VoxData) -> [i32; 3] {
//     let model_size = vox_data.models.first().unwrap().size;
//     [
//         model_size.x as i32,
//         model_size.z as i32,
//         model_size.y as i32,
//     ]
// }

// fn vox_data_to_structure_data(
//     vox_data: &VoxData,
// ) -> (Arc<Vec<Vec<Vec<BlockType>>>>, [i32; 3]) {
//     (
//         Arc::new(vox_data_to_blocks(vox_data)),
//         vox_data_model_size(vox_data),
//     )
// }
