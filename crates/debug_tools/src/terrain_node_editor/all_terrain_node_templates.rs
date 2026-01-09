use egui::{Color32, ColorImage, Context, TextureOptions};
use egui_node_editor::NodeTemplateIter;

use crate::terrain_node_editor::{
    noise_output_type::NoiseOutputType,
    terrain_node_template::TerrainNodeTemplate,
};

pub struct AllTerrainNodeTemplates<'a>(pub &'a Context, pub &'a mut TerrainUserState);
impl<'a> NodeTemplateIter for AllTerrainNodeTemplates<'a> {
    type Item = TerrainNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        let preview_texture = self.0.load_texture(
            "preview",
            ColorImage::filled([64, 64], Color32::RED),
            TextureOptions::NEAREST,
        );

        

        vec![
            TerrainNodeTemplate::Output(NoiseOutputType::TerrainHeight),
            TerrainNodeTemplate::SimplexNoise,
            TerrainNodeTemplate::NoiseAdd,
            TerrainNodeTemplate::NoiseSub,
            TerrainNodeTemplate::NoisePower,
            TerrainNodeTemplate::Constant,
            TerrainNodeTemplate::Multiply,
            TerrainNodeTemplate::MapRange,
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
            TerrainNodeTemplate::Preview(preview_texture.id()),
        ]
    }
}
