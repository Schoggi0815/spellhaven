use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::chunk_generation::{
    block_type::BlockType,
    chunk_lod::ChunkLod,
    structures::{
        oak_structure_generator::OakStructureGenerator,
        pine_structure_generator::PineStructureGenerator,
        structure_generator::{StructureGenerator, VoxelStructureMetadata},
    },
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum StructureGenerators {
    Oak(OakStructureGenerator),
    Pine(PineStructureGenerator),
}

impl StructureGenerator for StructureGenerators {
    fn get_structure_metadata(&self) -> &VoxelStructureMetadata {
        match self {
            Self::Oak(sg) => sg.get_structure_metadata(),
            Self::Pine(sg) => sg.get_structure_metadata(),
        }
    }

    fn get_structure_model(
        &self,
        structure_position: bevy::math::IVec2,
        lod: ChunkLod,
    ) -> Rc<Vec<Vec<Vec<BlockType>>>> {
        match self {
            Self::Oak(sg) => sg.get_structure_model(structure_position, lod),
            Self::Pine(sg) => sg.get_structure_model(structure_position, lod),
        }
    }
}
