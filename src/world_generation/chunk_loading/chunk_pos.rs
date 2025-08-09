use bevy::prelude::*;

use crate::world_generation::{
    chunk_generation::{
        CHUNK_SIZE, VOXEL_SIZE,
        chunk_lod::{ChunkLod, MAX_LOD},
    },
    chunk_loading::chunk_tree::ChunkTreePos,
};

#[derive(Deref, DerefMut, Clone, Copy, Hash, PartialEq, Eq)]
pub struct AbsoluteChunkPos(IVec2);

impl AbsoluteChunkPos {
    pub fn new(pos: IVec2) -> Self {
        Self(pos)
    }

    pub fn get_pos_center(&self) -> Vec2 {
        (self.0 * CHUNK_SIZE as i32 + (CHUNK_SIZE as i32 / 2)).as_vec2()
            * VOXEL_SIZE
    }

    pub fn from_absolute(absolute_pos: Vec3) -> Self {
        Self::new(
            (absolute_pos.xz() / VOXEL_SIZE / CHUNK_SIZE as f32)
                .floor()
                .as_ivec2(),
        )
    }

    pub fn to_absolute(&self, min_height: i32, chunk_lod: ChunkLod) -> Vec3 {
        let self_absolute = self.as_vec2() * VOXEL_SIZE * CHUNK_SIZE as f32;
        Vec3::new(
            self_absolute.x,
            min_height as f32 * VOXEL_SIZE * chunk_lod.multiplier_f32(),
            self_absolute.y,
        )
    }
}

impl From<IVec2> for AbsoluteChunkPos {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct RelativeChunkPos(IVec2);

impl RelativeChunkPos {
    pub fn new(pos: IVec2) -> Self {
        Self(pos)
    }

    pub fn to_absolute(&self, tree_pos: ChunkTreePos) -> AbsoluteChunkPos {
        AbsoluteChunkPos::new((**self) + (*tree_pos * MAX_LOD.multiplier_i32()))
    }
}

impl From<IVec2> for RelativeChunkPos {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}
