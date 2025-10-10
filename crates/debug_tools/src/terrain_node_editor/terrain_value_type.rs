use std::fmt::Debug;

use egui::DragValue;
use egui_node_editor::{NodeId, WidgetValueTrait};
use serde::{Deserialize, Serialize};

use crate::{
    terrain_node_editor::{
        terrain_node_data::TerrainNodeData, terrain_response::TerrainResponse,
    },
    world_generation::chunk_generation::noise::terrain_noise_type::{
        ConstantValue, TerrainNoiseType,
    },
};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TerrainValueType {
    NoiseF64x2 {
        value_or_index: ValueOrIndex<NoiseValue>,
    },
    F64 {
        value_or_index: ValueOrIndex<f64>,
    },
    I64 {
        value_or_index: ValueOrIndex<i64>,
    },
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct NoiseValue(pub f64);

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ValueOrIndex<T> {
    Value(T),
    Index(usize),
}

impl ValueOrIndex<i64> {
    pub fn get_index(&self, noise_array: &mut Vec<TerrainNoiseType>) -> usize {
        match self {
            ValueOrIndex::Index(index) => *index,
            ValueOrIndex::Value(value) => {
                let index = noise_array.len();
                noise_array.push(TerrainNoiseType::ConstantValue {
                    value: ConstantValue::I64(*value),
                });
                index
            }
        }
    }
}

impl ValueOrIndex<f64> {
    pub fn get_index(&self, noise_array: &mut Vec<TerrainNoiseType>) -> usize {
        match self {
            ValueOrIndex::Index(index) => *index,
            ValueOrIndex::Value(value) => {
                let index = noise_array.len();
                noise_array.push(TerrainNoiseType::ConstantValue {
                    value: ConstantValue::F64(*value),
                });
                index
            }
        }
    }
}

impl ValueOrIndex<NoiseValue> {
    pub fn get_index(&self, noise_array: &mut Vec<TerrainNoiseType>) -> usize {
        match self {
            ValueOrIndex::Index(index) => *index,
            ValueOrIndex::Value(value) => {
                let index = noise_array.len();
                noise_array.push(TerrainNoiseType::ConstantValue {
                    value: ConstantValue::F64(value.0),
                });
                noise_array
                    .push(TerrainNoiseType::Constant { value_index: index });
                index + 1
            }
        }
    }
}

impl TerrainValueType {
    pub fn get_noise_index(
        &self,
        noise_array: &mut Vec<TerrainNoiseType>,
    ) -> usize {
        match self {
            TerrainValueType::NoiseF64x2 { value_or_index } => {
                value_or_index.get_index(noise_array)
            }
            _ => 0,
        }
    }

    pub fn get_f64_index(
        &self,
        noise_array: &mut Vec<TerrainNoiseType>,
    ) -> usize {
        match self {
            TerrainValueType::F64 { value_or_index } => {
                value_or_index.get_index(noise_array)
            }
            _ => 0,
        }
    }

    pub fn get_i64_index(
        &self,
        noise_array: &mut Vec<TerrainNoiseType>,
    ) -> usize {
        match self {
            TerrainValueType::I64 { value_or_index } => {
                value_or_index.get_index(noise_array)
            }
            _ => 0,
        }
    }
}

impl Default for TerrainValueType {
    fn default() -> Self {
        Self::NoiseF64x2 {
            value_or_index: ValueOrIndex::Value(NoiseValue(0.)),
        }
    }
}

impl WidgetValueTrait for TerrainValueType {
    type Response = TerrainResponse;
    type UserState = ();
    type NodeData = TerrainNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut (),
        _node_data: &TerrainNodeData,
    ) -> Vec<Self::Response> {
        match self {
            TerrainValueType::NoiseF64x2 { value_or_index } => {
                ui.label(param_name);
                if let ValueOrIndex::Value(value) = value_or_index {
                    ui.add(DragValue::new(&mut value.0));
                }
            }
            TerrainValueType::F64 { value_or_index } => {
                ui.label(param_name);
                if let ValueOrIndex::Value(value) = value_or_index {
                    ui.add(DragValue::new(value));
                }
            }
            TerrainValueType::I64 { value_or_index } => {
                ui.label(param_name);
                if let ValueOrIndex::Value(value) = value_or_index {
                    ui.add(DragValue::new(value));
                }
            }
        }
        Vec::new()
    }
}
