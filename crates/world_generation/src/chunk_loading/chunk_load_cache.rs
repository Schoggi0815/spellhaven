use std::collections::HashMap;

use bevy::prelude::*;

use crate::chunk_loading::chunk_tree::ChunkTreePos;

#[derive(Resource, Default)]
pub struct ChunkLoadCache {
    pub tree_map: HashMap<ChunkTreePos, Entity>,
}
