use egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NoiseOutputType {
    TerrainHeight,
    // Oak
    OakMinThickness,
    OakMaxLength,
    OakMinLength,
    OakMaxAngle,
    OakStartThickness,
    OakStartXAngle,
    OakStartYAngle,
    // Pine
    PineStemPieceLength,
    PineStemThickness,
    PineStemCount,
    PineBranchPieceLenght,
    PineBranchDownAngle,
    PineBranchSpiral,
    PineBranchDroop,
    PineNeedleAngleOffset,
}

pub const ALL_NOISE_OUTPUT_TYPES: &[NoiseOutputType] = &[
    NoiseOutputType::TerrainHeight,
    NoiseOutputType::OakMinThickness,
    NoiseOutputType::OakMaxLength,
    NoiseOutputType::OakMinLength,
    NoiseOutputType::OakMaxAngle,
    NoiseOutputType::OakStartThickness,
    NoiseOutputType::OakStartXAngle,
    NoiseOutputType::OakStartYAngle,
    NoiseOutputType::PineStemPieceLength,
    NoiseOutputType::PineStemThickness,
    NoiseOutputType::PineStemCount,
    NoiseOutputType::PineBranchPieceLenght,
    NoiseOutputType::PineBranchDownAngle,
    NoiseOutputType::PineBranchSpiral,
    NoiseOutputType::PineBranchDroop,
    NoiseOutputType::PineNeedleAngleOffset,
];

impl NoiseOutputType {
    pub fn get_text(&self) -> &'static str {
        match self {
            NoiseOutputType::TerrainHeight => "Terrain Height",
            NoiseOutputType::OakMinThickness => "Oak Min Thickness",
            NoiseOutputType::OakMaxLength => "Oak Max Length",
            NoiseOutputType::OakMinLength => "Oak Min Length",
            NoiseOutputType::OakMaxAngle => "Oak Max Angle",
            NoiseOutputType::OakStartThickness => "Oak Start Thickness",
            NoiseOutputType::OakStartXAngle => "Oak Start X Angle",
            NoiseOutputType::OakStartYAngle => "Oak Start Y Angle",
            NoiseOutputType::PineStemPieceLength => "Pine Stem Piece Length",
            NoiseOutputType::PineStemThickness => "Pine Stem Thickness",
            NoiseOutputType::PineStemCount => "Pine Stem Count",
            NoiseOutputType::PineBranchPieceLenght => {
                "Pine Branch Piece Length"
            }
            NoiseOutputType::PineBranchDownAngle => "Pine Branch Down Angle",
            NoiseOutputType::PineBranchSpiral => "Pine Branch Spiral",
            NoiseOutputType::PineBranchDroop => "Pine Branch Droop",
            NoiseOutputType::PineNeedleAngleOffset => {
                "Pine Needle Angle Offset"
            }
        }
    }

    pub fn render_selectable_value(
        &self,
        ui: &mut Ui,
        current: &mut NoiseOutputType,
    ) {
        ui.selectable_value(current, *self, self.get_text());
    }
}
