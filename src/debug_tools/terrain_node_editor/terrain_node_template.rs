use std::borrow::Cow;

use egui_node_editor::{Graph, InputParamKind, NodeTemplateTrait};
use serde::{Deserialize, Serialize};

use crate::{
    debug_tools::terrain_node_editor::{
        terrain_data_type::TerrainDataType,
        terrain_node_data::TerrainNodeData,
        terrain_value_type::{NoiseValue, TerrainValueType, ValueOrIndex},
    },
    world_generation::chunk_generation::noise::gradient_fractal_noise::{
        DEFAULT_AMPLITUDE, DEFAULT_FREQUENCY, DEFAULT_GRADIENT,
        DEFAULT_LACUNARITY, DEFAULT_OCTAVE_COUNT, DEFAULT_PERSISTENCE,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TerrainNodeTemplate {
    Output,
    SimplexNoise,
    NoiseAdd,
    NoiseSub,
    NoisePower,
    Constant,
    Multiply,
    SmoothStep,
    ScalePoint,
    GFT,
    Max,
    Abs,
    TranslatePoint,
    PowF64,
    VoxelSize,
    RandomI64,
    RandomF64,
    DivideF64,
}

pub type TerrainGraph =
    Graph<TerrainNodeData, TerrainDataType, TerrainValueType>;

impl NodeTemplateTrait for TerrainNodeTemplate {
    type NodeData = TerrainNodeData;

    type DataType = TerrainDataType;

    type ValueType = TerrainValueType;

    type UserState = ();

    type CategoryType = &'static str;

    fn node_finder_label(
        &self,
        _: &mut Self::UserState,
    ) -> std::borrow::Cow<str> {
        Cow::Borrowed(match self {
            TerrainNodeTemplate::Output => "Output",
            TerrainNodeTemplate::SimplexNoise => "Simplex Noise",
            TerrainNodeTemplate::NoiseAdd => "Noise Add",
            TerrainNodeTemplate::NoisePower => "Noise Power",
            TerrainNodeTemplate::PowF64 => "Power F64",
            TerrainNodeTemplate::Constant => "Constant Noise",
            TerrainNodeTemplate::Multiply => "Multiply Noise",
            TerrainNodeTemplate::SmoothStep => "Smooth Step",
            TerrainNodeTemplate::ScalePoint => "Scale Point",
            TerrainNodeTemplate::GFT => "Gradient Fractal Noise",
            TerrainNodeTemplate::Max => "Max",
            TerrainNodeTemplate::Abs => "Abs",
            TerrainNodeTemplate::TranslatePoint => "Translate Point",
            TerrainNodeTemplate::VoxelSize => "Voxel Size",
            TerrainNodeTemplate::RandomI64 => "Random Integer",
            TerrainNodeTemplate::RandomF64 => "Random Float",
            TerrainNodeTemplate::DivideF64 => "Divide",
            TerrainNodeTemplate::NoiseSub => "Noise Sub",
        })
    }

    fn node_finder_categories(
        &self,
        _user_state: &mut Self::UserState,
    ) -> Vec<&'static str> {
        match self {
            TerrainNodeTemplate::Output => vec![],
            TerrainNodeTemplate::SimplexNoise => vec!["Noise Functions"],
            TerrainNodeTemplate::NoiseAdd
            | TerrainNodeTemplate::NoiseSub
            | TerrainNodeTemplate::NoisePower
            | TerrainNodeTemplate::Constant
            | TerrainNodeTemplate::Multiply
            | TerrainNodeTemplate::SmoothStep
            | TerrainNodeTemplate::ScalePoint
            | TerrainNodeTemplate::GFT
            | TerrainNodeTemplate::Max
            | TerrainNodeTemplate::Abs
            | TerrainNodeTemplate::TranslatePoint => {
                vec!["Noise Calculations"]
            }
            TerrainNodeTemplate::PowF64
            | TerrainNodeTemplate::VoxelSize
            | TerrainNodeTemplate::DivideF64
            | TerrainNodeTemplate::RandomF64 => {
                vec!["F64"]
            }
            TerrainNodeTemplate::RandomI64 => vec!["I64"],
        }
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _: &mut Self::UserState) -> Self::NodeData {
        TerrainNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut egui_node_editor::Graph<
            Self::NodeData,
            Self::DataType,
            Self::ValueType,
        >,
        _user_state: &mut Self::UserState,
        node_id: egui_node_editor::NodeId,
    ) {
        let input_noise = |graph: &mut TerrainGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                TerrainDataType::NoiseF64x2,
                TerrainValueType::NoiseF64x2 {
                    value_or_index: ValueOrIndex::Value(NoiseValue(0.)),
                },
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let input_f64_with_default =
            |graph: &mut TerrainGraph, name: &str, default: f64| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    TerrainDataType::F64,
                    TerrainValueType::F64 {
                        value_or_index: ValueOrIndex::Value(default),
                    },
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            };
        let input_f64 = |graph: &mut TerrainGraph, name: &str| {
            input_f64_with_default(graph, name, 0.0)
        };
        let input_i64_with_default =
            |graph: &mut TerrainGraph, name: &str, default: i64| {
                graph.add_input_param(
                    node_id,
                    name.to_string(),
                    TerrainDataType::I64,
                    TerrainValueType::I64 {
                        value_or_index: ValueOrIndex::Value(default),
                    },
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
            };
        let input_i64 = |graph: &mut TerrainGraph, name: &str| {
            input_i64_with_default(graph, name, 0)
        };

        let output_noise = |graph: &mut TerrainGraph, name: &str| {
            graph.add_output_param(
                node_id,
                name.to_string(),
                TerrainDataType::NoiseF64x2,
            );
        };
        let output_f64 = |graph: &mut TerrainGraph, name: &str| {
            graph.add_output_param(
                node_id,
                name.to_string(),
                TerrainDataType::F64,
            );
        };
        let output_i64 = |graph: &mut TerrainGraph, name: &str| {
            graph.add_output_param(
                node_id,
                name.to_string(),
                TerrainDataType::I64,
            );
        };

        match self {
            TerrainNodeTemplate::Output => {
                input_noise(graph, "A");
            }
            TerrainNodeTemplate::SimplexNoise => {
                input_i64(graph, "seed");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::NoiseAdd => {
                input_noise(graph, "A");
                input_noise(graph, "B");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::NoiseSub => {
                input_noise(graph, "A");
                input_noise(graph, "B");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::NoisePower => {
                input_noise(graph, "A");
                input_noise(graph, "B");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::PowF64 => {
                input_f64(graph, "A");
                input_f64(graph, "B");
                output_f64(graph, "out");
            }
            TerrainNodeTemplate::Constant => {
                input_f64(graph, "A");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::Multiply => {
                input_noise(graph, "A");
                input_noise(graph, "B");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::Max => {
                input_noise(graph, "A");
                input_noise(graph, "B");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::TranslatePoint => {
                input_f64(graph, "X");
                input_f64(graph, "Y");
                input_noise(graph, "noise");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::SmoothStep => {
                input_noise(graph, "noise");
                input_f64(graph, "steps");
                input_f64(graph, "smoothness");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::ScalePoint => {
                input_noise(graph, "noise");
                input_f64(graph, "scale");
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::GFT => {
                input_noise(graph, "noise");
                input_i64_with_default(
                    graph,
                    "octaves",
                    DEFAULT_OCTAVE_COUNT as i64,
                );
                input_f64_with_default(graph, "frequency", DEFAULT_FREQUENCY);
                input_f64_with_default(graph, "lacunarity", DEFAULT_LACUNARITY);
                input_f64_with_default(
                    graph,
                    "persistence",
                    DEFAULT_PERSISTENCE,
                );
                input_f64_with_default(graph, "gradient", DEFAULT_GRADIENT);
                input_f64_with_default(graph, "amplitude", DEFAULT_AMPLITUDE);
                output_noise(graph, "out");
            }
            TerrainNodeTemplate::VoxelSize => output_f64(graph, "out"),
            TerrainNodeTemplate::RandomI64 => output_i64(graph, "random int"),
            TerrainNodeTemplate::DivideF64 => {
                input_f64(graph, "A");
                input_f64(graph, "B");
                output_f64(graph, "out");
            }
            TerrainNodeTemplate::RandomF64 => {
                input_f64(graph, "min");
                input_f64(graph, "max");
                output_f64(graph, "random float");
            }
            TerrainNodeTemplate::Abs => {
                input_noise(graph, "noise");
                output_noise(graph, "out");
            }
        }
    }
}
