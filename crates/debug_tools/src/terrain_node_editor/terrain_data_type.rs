use std::borrow::Cow;

use egui::Color32;
use egui_node_editor::DataTypeTrait;
use serde::{Deserialize, Serialize};

use crate::terrain_node_editor::terrain_graph_state::TerrainGraphState;

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainDataType {
    NoiseF64x2,
    F64,
    I64,
}

impl DataTypeTrait<TerrainGraphState> for TerrainDataType {
    fn data_type_color(&self, _: &mut TerrainGraphState) -> Color32 {
        match self {
            TerrainDataType::NoiseF64x2 => Color32::YELLOW,
            TerrainDataType::F64 => Color32::BLUE,
            TerrainDataType::I64 => Color32::DARK_GREEN,
        }
    }

    fn name(&self) -> std::borrow::Cow<'_, str> {
        match self {
            TerrainDataType::NoiseF64x2 => Cow::Borrowed("Noise F64 x 2"),
            TerrainDataType::F64 => Cow::Borrowed("F64"),
            TerrainDataType::I64 => Cow::Borrowed("I64"),
        }
    }
}
