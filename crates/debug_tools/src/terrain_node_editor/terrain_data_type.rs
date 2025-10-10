use std::borrow::Cow;

use bevy_inspector_egui::egui::Color32;
use egui_node_editor::DataTypeTrait;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainDataType {
    NoiseF64x2,
    F64,
    I64,
}

impl DataTypeTrait<()> for TerrainDataType {
    fn data_type_color(&self, _: &mut ()) -> Color32 {
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
