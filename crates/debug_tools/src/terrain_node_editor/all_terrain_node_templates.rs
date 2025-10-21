use egui_node_editor::NodeTemplateIter;

use crate::terrain_node_editor::terrain_node_template::TerrainNodeTemplate;

pub struct AllTerrainNodeTemplates;
impl NodeTemplateIter for AllTerrainNodeTemplates {
    type Item = TerrainNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![
            TerrainNodeTemplate::Output,
            TerrainNodeTemplate::SimplexNoise,
            TerrainNodeTemplate::NoiseAdd,
            TerrainNodeTemplate::NoiseSub,
            TerrainNodeTemplate::Constant,
            TerrainNodeTemplate::Multiply,
            TerrainNodeTemplate::SmoothStep,
            TerrainNodeTemplate::ScalePoint,
            TerrainNodeTemplate::GFT,
            TerrainNodeTemplate::Max,
            TerrainNodeTemplate::Abs,
            TerrainNodeTemplate::TranslatePoint,
            TerrainNodeTemplate::PowF64,
            TerrainNodeTemplate::VoxelSize,
            TerrainNodeTemplate::DivideF64,
            TerrainNodeTemplate::RandomI64,
            TerrainNodeTemplate::RandomF64,
        ]
    }
}
