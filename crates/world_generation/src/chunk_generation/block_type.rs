use bevy::color::{Color, Mix};
use serde::{Deserialize, Serialize};

use crate::chunk_generation::mesh_type::MeshType;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum BlockType {
    Air,
    Stone,
    Grass(u8),
    Log,
    PineLog,
    Snow,
    Leaf,
    PineNeedle,
    Dirt,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum BlockFace {
    Top,
    Bottom,
    Front,
    Back,
    Right,
    Left,
}

impl BlockType {
    pub fn is_covering_for(&self, other: &BlockType) -> bool {
        if self == other {
            return true;
        }

        match self {
            BlockType::Air => false,
            _ => true,
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            BlockType::Air => Color::NONE,
            BlockType::Stone => Color::linear_rgb(0.2, 0.2, 0.2),
            // BlockType::Grass(hue) => Color::oklch(0.7764, 0.1708, 65.16)
            //     .mix(&Color::oklch(0.7764, 0.146, 227.64), *hue as f32 / 255.),
            BlockType::Grass(hue) => Color::hsl(33., 0.7, 0.5)
                .mix(&Color::hsl(170., 0.66, 0.5), *hue as f32 / 255.),
            BlockType::Log => Color::linear_rgb(0.3, 0.15, 0.0),
            BlockType::Snow => Color::linear_rgb(0.9, 0.9, 0.9),
            BlockType::Leaf => Color::linear_rgb(0.2, 0.5, 0.2),
            BlockType::Dirt => Color::linear_rgb(0.3, 0.2, 0.0),
            BlockType::PineLog => Color::linear_rgb(0.0075, 0.002, 0.0),
            BlockType::PineNeedle => Color::linear_rgb(0.003, 0.015, 0.002),
        }
    }

    pub fn get_mesh_type(&self) -> MeshType {
        match self {
            _ => MeshType::Opaque,
        }
    }

    pub fn is_colliding(&self) -> bool {
        match self {
            BlockType::Air => false,
            _ => true,
        }
    }
}
