use bevy::math::IVec3;
use serde::{Deserialize, Serialize};

use crate::chunk_generation::block_type::BlockType;

#[derive(Serialize, Deserialize)]
pub struct StructureModel {
    pub blocks: Vec<Vec<Vec<BlockType>>>,
    pub model_size: IVec3,
}
