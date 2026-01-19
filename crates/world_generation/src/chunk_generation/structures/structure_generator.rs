use bevy::math::IVec2;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use crate::chunk_generation::{
    block_type::BlockType,
    chunk_lod::ChunkLod,
    noise::terrain_noise::TerrainNoise,
    structures::{
        noise_wrapper::NoiseWrapper, structure_generators::StructureGenerators,
    },
};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct VoxelStructureMetadata {
    pub model_size: [i32; 3],
    pub generation_size: [i32; 2],
    pub grid_offset: [i32; 2],
    pub generate_debug_blocks: bool,
    pub debug_rgb_multiplier: [f32; 3],
    pub noise: NoiseWrapper,
}

impl VoxelStructureMetadata {
    pub fn new(
        model_size: [i32; 3],
        generation_size: [i32; 2],
        grid_offset: [i32; 2],
        noise_map: TerrainNoise,
        seed: u64,
    ) -> Self {
        Self {
            model_size,
            generation_size,
            grid_offset,
            generate_debug_blocks: false,
            debug_rgb_multiplier: [0., 0., 0.],
            noise: NoiseWrapper::new(noise_map, seed),
        }
    }

    pub fn with_debug_blocks(self, generate_debug_blocks: bool) -> Self {
        Self {
            generate_debug_blocks,
            ..self
        }
    }

    pub fn with_debug_rgb_multiplier(
        self,
        debug_rgb_multiplier: [f32; 3],
    ) -> Self {
        Self {
            debug_rgb_multiplier,
            ..self
        }
    }
}

pub trait StructureGenerator {
    fn get_structure_metadata(&self) -> &VoxelStructureMetadata;
    fn get_structure_model(
        &self,
        structure_position: IVec2,
        lod: ChunkLod,
    ) -> Rc<Vec<Vec<Vec<BlockType>>>>;
}

pub struct FixedStructureGenerator {
    pub fixed_structure_metadata: VoxelStructureMetadata,
    pub fixed_structure_model: Arc<Vec<Vec<Vec<BlockType>>>>,
}

impl StructureGenerator for FixedStructureGenerator {
    fn get_structure_metadata(&self) -> &VoxelStructureMetadata {
        &self.fixed_structure_metadata
    }

    fn get_structure_model(
        &self,
        _: IVec2,
        _: ChunkLod,
    ) -> Rc<Vec<Vec<Vec<BlockType>>>> {
        Rc::new(self.fixed_structure_model.to_vec())
    }
}

pub struct StructureGeneratorCache {
    cache: RefCell<HashMap<IVec2, Rc<Vec<Vec<Vec<BlockType>>>>>>,
    structure_generator: Arc<Box<StructureGenerators>>,
}

impl StructureGeneratorCache {
    pub fn new(structure_generator: &Arc<Box<StructureGenerators>>) -> Self {
        Self {
            structure_generator: structure_generator.clone(),
            cache: RefCell::new(HashMap::new()),
        }
    }
}

impl StructureGenerator for StructureGeneratorCache {
    fn get_structure_metadata(&self) -> &VoxelStructureMetadata {
        self.structure_generator.get_structure_metadata()
    }

    fn get_structure_model(
        &self,
        structure_position: IVec2,
        lod: ChunkLod,
    ) -> Rc<Vec<Vec<Vec<BlockType>>>> {
        let structure_position = if lod.usize() >= ChunkLod::Eighth.usize() {
            IVec2::new(0, 0)
        } else {
            structure_position
        };

        let mut cache = self.cache.borrow_mut();
        if let Some(model) = cache.get(&structure_position) {
            return model.clone();
        }

        let model = self
            .structure_generator
            .get_structure_model(structure_position, lod);

        cache.insert(structure_position, model.clone());

        model
    }
}
