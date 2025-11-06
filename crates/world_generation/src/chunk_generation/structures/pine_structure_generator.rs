use bevy::prelude::*;
use rand::rngs::StdRng;

use crate::chunk_generation::{
    block_type::BlockType,
    structures::{
        foliage_generation::{
            pine_l_system::PineLSystem, tree_l_system::LSystem,
        },
        structure_generator::VoxelStructureMetadata,
        tree_structure_generator::TreeStructureGenerator,
    },
};

pub struct PineStructureGenerator {
    pub fixed_structure_metadata: VoxelStructureMetadata,
}

impl TreeStructureGenerator for PineStructureGenerator {
    fn new(mut metadata: VoxelStructureMetadata, _: &mut StdRng) -> Self {
        Self::adjust_metadata(&mut metadata);

        Self {
            fixed_structure_metadata: metadata,
        }
    }

    fn get_structure_metadata(&self) -> &VoxelStructureMetadata {
        &self.fixed_structure_metadata
    }

    fn grow(
        &self,
        rng: &mut StdRng,
        _structure_position: IVec2,
    ) -> Vec<Vec<Vec<BlockType>>> {
        PineLSystem::grow_new(
            rng,
            &(),
            IVec3::from_array(self.fixed_structure_metadata.model_size)
                .as_usizevec3(),
        )
    }
}
