use serde::{Deserialize, Serialize};

use crate::chunk_generation::noise::terrain_noise::TerrainNoise;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TerrainNoiseGroup {
    pub terrain_height: TerrainNoise,
    pub grass_hue: TerrainNoise,
    // Oak
    pub oak_min_thickness: TerrainNoise,
    pub oak_max_length: TerrainNoise,
    pub oak_min_length: TerrainNoise,
    pub oak_max_angle: TerrainNoise,
    pub oak_start_thickness: TerrainNoise,
    pub oak_start_x_angle: TerrainNoise,
    pub oak_start_y_angle: TerrainNoise,
    // Pine
    pub pine_stem_piece_length: TerrainNoise,
    pub pine_stem_thickness: TerrainNoise,
    pub pine_stem_count: TerrainNoise,
    pub pine_branch_piece_lenght: TerrainNoise,
    pub pine_branch_down_angle: TerrainNoise,
    pub pine_branch_spiral: TerrainNoise,
    pub pine_branch_droop: TerrainNoise,
    pub pine_needle_angle_offset: TerrainNoise,
}
