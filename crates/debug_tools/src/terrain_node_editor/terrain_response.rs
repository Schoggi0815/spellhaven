use egui_node_editor::{NodeId, UserResponseTrait};

use crate::terrain_node_editor::noise_output_type::NoiseOutputType;

#[derive(Debug, Clone)]
pub enum TerrainResponse {
    UpdateOutputType(NodeId, NoiseOutputType),
    SetPreviewNode(NodeId),
}

impl UserResponseTrait for TerrainResponse {}
