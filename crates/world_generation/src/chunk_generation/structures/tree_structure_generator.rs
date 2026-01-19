use std::rc::Rc;

use bevy::math::IVec2;
use rand::{SeedableRng, rngs::StdRng};

use crate::chunk_generation::{
    VOXEL_SIZE,
    block_type::BlockType,
    chunk_lod::ChunkLod,
    noise::terrain_noise_group::TerrainNoiseGroup,
    structures::structure_generator::{
        StructureGenerator, VoxelStructureMetadata,
    },
};

pub trait TreeStructureGenerator {
    fn adjust_metadata(metadata: &mut VoxelStructureMetadata) {
        let model_size = [
            (metadata.model_size[0] as f32 / VOXEL_SIZE) as i32,
            (metadata.model_size[1] as f32 / VOXEL_SIZE) as i32,
            (metadata.model_size[2] as f32 / VOXEL_SIZE) as i32,
        ];

        let generation_size = [
            (metadata.generation_size[0] as f32 / VOXEL_SIZE) as i32,
            (metadata.generation_size[1] as f32 / VOXEL_SIZE) as i32,
        ];

        let grid_offset = [
            (metadata.grid_offset[0] as f32 / VOXEL_SIZE) as i32,
            (metadata.grid_offset[1] as f32 / VOXEL_SIZE) as i32,
        ];

        metadata.model_size = model_size;
        metadata.generation_size = generation_size;
        metadata.grid_offset = grid_offset;
    }

    fn new(
        metadata: VoxelStructureMetadata,
        noise_group: &TerrainNoiseGroup,
        world_seed: u64,
    ) -> Self;
    fn get_structure_metadata(&self) -> &VoxelStructureMetadata;
    fn grow(
        &self,
        rng: &mut StdRng,
        structure_position: IVec2,
    ) -> Vec<Vec<Vec<BlockType>>>;
}

impl<T: TreeStructureGenerator> StructureGenerator for T {
    fn get_structure_metadata(&self) -> &VoxelStructureMetadata {
        return &self.get_structure_metadata();
    }

    fn get_structure_model(
        &self,
        structure_position: IVec2,
        _: ChunkLod,
    ) -> Rc<Vec<Vec<Vec<BlockType>>>> {
        let noise_value = self
            .get_structure_metadata()
            .noise
            .get(structure_position.as_dvec2().to_array())
            * 0.5
            + 0.5;
        let mut rng =
            StdRng::seed_from_u64((noise_value.value.abs() * 10000.) as u64);

        let voxel_grid = Self::grow(&self, &mut rng, structure_position);

        Rc::new(voxel_grid)
    }
}
