use serde::{Deserialize, Serialize};

use crate::world_generation::chunk_generation::structures::{
    oak_structure_generator::OakStructureGenerator,
    structure_generator::StructureGenerator,
};

#[derive(Clone, Serialize, Deserialize)]
pub enum StructureGenerators {
    Oak(OakStructureGenerator),
}

impl StructureGenerator for StructureGenerators {
    fn get_structure_metadata(
        &self,
    ) -> &super::structure_generator::VoxelStructureMetadata {
        match self {
            Self::Oak(sg) => sg.get_structure_metadata(),
        }
    }

    fn get_structure_model(
        &self,
        structure_position: bevy::math::IVec2,
        lod: crate::world_generation::chunk_generation::chunk_lod::ChunkLod,
    ) -> std::rc::Rc<Vec<Vec<Vec<crate::world_generation::chunk_generation::block_type::BlockType>>>>{
        match self {
            Self::Oak(sg) => sg.get_structure_model(structure_position, lod),
        }
    }
}
