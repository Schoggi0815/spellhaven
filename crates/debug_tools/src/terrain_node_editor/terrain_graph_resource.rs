use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use anyhow::anyhow;
use bevy::prelude::*;
use egui::Ui;
use egui_node_editor::{GraphEditorState, GraphResponse, Node, OutputId};
use ron::ser::PrettyConfig;

use crate::{
    terrain_node_editor::{
        all_terrain_node_templates::AllTerrainNodeTemplates,
        terrain_data_type::TerrainDataType,
        terrain_node_data::TerrainNodeData,
        terrain_node_template::{TerrainGraph, TerrainNodeTemplate},
        terrain_response::TerrainResponse,
        terrain_value_type::{TerrainValueType, ValueOrIndex},
    },
    world_generation::chunk_generation::noise::{
        terrain_noise::{TerrainNoise, TERRAIN_NOISE_FILE_PATH},
        terrain_noise_type::TerrainNoiseType,
    },
};

type TerrainGraphState = GraphEditorState<
    TerrainNodeData,
    TerrainDataType,
    TerrainValueType,
    TerrainNodeTemplate,
    (),
>;

#[derive(Resource, Deref, DerefMut)]
pub struct TerrainGraphResource {
    state: TerrainGraphState,
}

const TERRAIN_NOISE_GRAPH_FILE_PATH: &'static str =
    "assets/terrain_noise_graph.ron";

impl Default for TerrainGraphResource {
    fn default() -> Self {
        let file = File::open(TERRAIN_NOISE_GRAPH_FILE_PATH);
        let Ok(mut file) = file else {
            return Self {
                state: Default::default(),
            };
        };

        let mut output_string = String::new();
        let read_result = file.read_to_string(&mut output_string);
        if read_result.is_err() {
            return Self {
                state: Default::default(),
            };
        }

        let state: TerrainGraphState =
            ron::from_str(&output_string).expect("cannot read terrain data");

        Self { state }
    }
}

impl TerrainGraphResource {
    pub fn draw(
        &mut self,
        ui: &mut Ui,
    ) -> GraphResponse<TerrainResponse, TerrainNodeData> {
        self.state.draw_graph_editor(
            ui,
            AllTerrainNodeTemplates,
            &mut (),
            Vec::default(),
        )
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let output_nodes = self
            .graph
            .nodes
            .iter()
            .filter(|node| {
                node.1.user_data.template == TerrainNodeTemplate::Output
            })
            .collect::<Vec<_>>();

        if output_nodes.len() != 1 {
            return Err(anyhow!(
                "Too many or too little number of output nodes!"
            ));
        }

        let output_node = output_nodes.first();
        let Some(output_node) = output_node else {
            return Err(anyhow!("No output node found!"));
        };

        let mut noise_array = Vec::new();
        let mut cache = HashMap::new();

        let start_index = get_terrain_noise_index(
            output_node.1,
            &mut noise_array,
            &self.graph,
            &mut cache,
        );

        let terrain_noise = TerrainNoise::new(
            start_index.get_noise_index(&mut noise_array),
            noise_array,
        );

        let mut file = File::create(TERRAIN_NOISE_FILE_PATH)?;
        let text = ron::ser::to_string_pretty(
            &terrain_noise,
            PrettyConfig::default(),
        )?;
        file.write_all(text.as_bytes())?;
        file.flush()?;

        let mut file = File::create(TERRAIN_NOISE_GRAPH_FILE_PATH)?;
        let text =
            ron::ser::to_string_pretty(&self.state, PrettyConfig::default())?;
        file.write_all(text.as_bytes())?;
        file.flush()?;

        Ok(())
    }
}

fn get_terrain_noise_index(
    node: &Node<TerrainNodeData>,
    noise_array: &mut Vec<TerrainNoiseType>,
    graph: &TerrainGraph,
    value_cache: &mut HashMap<OutputId, TerrainValueType>,
) -> TerrainValueType {
    let mut get_input_value = |input_name: &str| {
        let input_id = node.get_input(input_name).expect("Input not found!");
        let connection = graph.connection(input_id);

        let Some(connection) = connection else {
            return graph[input_id].value;
        };

        if let Some(cached_value) = value_cache.get(&connection) {
            return *cached_value;
        }

        let node = &graph[graph[connection].node];
        let value =
            get_terrain_noise_index(node, noise_array, graph, value_cache);
        value_cache.insert(connection, value);
        value
    };

    match node.user_data.template {
        TerrainNodeTemplate::Output => get_input_value("A"),
        TerrainNodeTemplate::SimplexNoise => {
            let seed_index = get_input_value("seed").get_i64_index(noise_array);
            let noise_index = noise_array.len();
            noise_array.push(TerrainNoiseType::Simplex { seed_index });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(noise_index),
            }
        }
        TerrainNodeTemplate::NoiseAdd => {
            let a_input = get_input_value("A");
            let b_input = get_input_value("B");
            let a_index = a_input.get_noise_index(noise_array);
            let b_index = b_input.get_noise_index(noise_array);
            let noise_index = noise_array.len();
            noise_array.push(TerrainNoiseType::Add { a_index, b_index });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(noise_index),
            }
        }
        TerrainNodeTemplate::NoiseSub => {
            let a_input = get_input_value("A");
            let b_input = get_input_value("B");
            let a_index = a_input.get_noise_index(noise_array);
            let b_index = b_input.get_noise_index(noise_array);
            let noise_index = noise_array.len();
            noise_array.push(TerrainNoiseType::Sub { a_index, b_index });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(noise_index),
            }
        }
        TerrainNodeTemplate::NoisePower => {
            let a_input = get_input_value("A");
            let b_input = get_input_value("B");
            let a_index = a_input.get_noise_index(noise_array);
            let b_index = b_input.get_noise_index(noise_array);
            let noise_index = noise_array.len();
            noise_array.push(TerrainNoiseType::Power { a_index, b_index });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(noise_index),
            }
        }
        TerrainNodeTemplate::PowF64 => {
            let a_input = get_input_value("A");
            let b_input = get_input_value("B");
            let a_index = a_input.get_f64_index(noise_array);
            let b_index = b_input.get_f64_index(noise_array);
            let index = noise_array.len();
            noise_array.push(TerrainNoiseType::Powf64 { a_index, b_index });
            TerrainValueType::F64 {
                value_or_index: ValueOrIndex::Index(index),
            }
        }
        TerrainNodeTemplate::Constant => {
            let a_index = get_input_value("A").get_f64_index(noise_array);
            let noise_index = noise_array.len();
            noise_array.push(TerrainNoiseType::Constant {
                value_index: a_index,
            });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(noise_index),
            }
        }
        TerrainNodeTemplate::Multiply => {
            let a_input = get_input_value("A");
            let b_input = get_input_value("B");
            let a_index = a_input.get_noise_index(noise_array);
            let b_index = b_input.get_noise_index(noise_array);
            let noise_index = noise_array.len();
            noise_array.push(TerrainNoiseType::Multiply { a_index, b_index });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(noise_index),
            }
        }
        TerrainNodeTemplate::Max => {
            let a_input = get_input_value("A");
            let b_input = get_input_value("B");
            let a_index = a_input.get_noise_index(noise_array);
            let b_index = b_input.get_noise_index(noise_array);
            let noise_index = noise_array.len();
            noise_array.push(TerrainNoiseType::Max { a_index, b_index });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(noise_index),
            }
        }
        TerrainNodeTemplate::Abs => {
            let input_index =
                get_input_value("noise").get_noise_index(noise_array);
            let noise_index = noise_array.len();
            noise_array.push(TerrainNoiseType::Abs { input_index });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(noise_index),
            }
        }
        TerrainNodeTemplate::SmoothStep => {
            let noise_input = get_input_value("noise");
            let steps_input = get_input_value("steps");
            let smoothness_input = get_input_value("smoothness");
            let noise_index = noise_input.get_noise_index(noise_array);
            let steps_index = steps_input.get_f64_index(noise_array);
            let smoothness_index = smoothness_input.get_f64_index(noise_array);
            let index = noise_array.len();
            noise_array.push(TerrainNoiseType::SmoothStep {
                noise_index,
                steps_index,
                smoothness_index,
            });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(index),
            }
        }
        TerrainNodeTemplate::ScalePoint => {
            let noise_input = get_input_value("noise");
            let scale_input = get_input_value("scale");
            let noise_index = noise_input.get_noise_index(noise_array);
            let scale_index = scale_input.get_f64_index(noise_array);
            let index = noise_array.len();
            noise_array.push(TerrainNoiseType::ScalePoint {
                noise_index,
                scale_index,
            });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(index),
            }
        }
        TerrainNodeTemplate::TranslatePoint => {
            let noise_input = get_input_value("noise");
            let x_input = get_input_value("X");
            let y_input = get_input_value("Y");
            let noise_index = noise_input.get_noise_index(noise_array);
            let x_index = x_input.get_f64_index(noise_array);
            let y_index = y_input.get_f64_index(noise_array);
            let index = noise_array.len();
            noise_array.push(TerrainNoiseType::TranslatePoint {
                noise_index,
                x_index,
                y_index,
            });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(index),
            }
        }
        TerrainNodeTemplate::GFT => {
            let noise_input = get_input_value("noise");
            let octaves_input = get_input_value("octaves");
            let frequency_input = get_input_value("frequency");
            let lacunarity_input = get_input_value("lacunarity");
            let persistance_input = get_input_value("persistence");
            let gradient_input = get_input_value("gradient");
            let amplitude_input = get_input_value("amplitude");
            let noise_index = noise_input.get_noise_index(noise_array);
            let octaves_index = octaves_input.get_i64_index(noise_array);
            let frequency_index = frequency_input.get_f64_index(noise_array);
            let lacunarity_index = lacunarity_input.get_f64_index(noise_array);
            let persistence_index =
                persistance_input.get_f64_index(noise_array);
            let gradient_index = gradient_input.get_f64_index(noise_array);
            let amplitude_index = amplitude_input.get_f64_index(noise_array);
            let index = noise_array.len();
            noise_array.push(TerrainNoiseType::GFT {
                noise_index,
                octaves_index,
                frequency_index,
                lacunarity_index,
                persistence_index,
                gradient_index,
                amplitude_index,
            });
            TerrainValueType::NoiseF64x2 {
                value_or_index: ValueOrIndex::Index(index),
            }
        }
        TerrainNodeTemplate::VoxelSize => {
            let index = noise_array.len();
            noise_array.push(TerrainNoiseType::VoxelSize);
            TerrainValueType::F64 {
                value_or_index: ValueOrIndex::Index(index),
            }
        }
        TerrainNodeTemplate::RandomI64 => {
            let index = noise_array.len();
            noise_array.push(TerrainNoiseType::RandomI64);
            TerrainValueType::I64 {
                value_or_index: ValueOrIndex::Index(index),
            }
        }
        TerrainNodeTemplate::DivideF64 => {
            let a_input = get_input_value("A");
            let b_input = get_input_value("B");
            let a_index = a_input.get_f64_index(noise_array);
            let b_index = b_input.get_f64_index(noise_array);
            let index = noise_array.len();
            noise_array.push(TerrainNoiseType::Dividef64 { a_index, b_index });
            TerrainValueType::F64 {
                value_or_index: ValueOrIndex::Index(index),
            }
        }
        TerrainNodeTemplate::RandomF64 => {
            let min_input = get_input_value("min");
            let max_input = get_input_value("max");
            let min_index = min_input.get_f64_index(noise_array);
            let max_index = max_input.get_f64_index(noise_array);
            let index = noise_array.len();
            noise_array.push(TerrainNoiseType::RandomF64 {
                min_index,
                max_index,
            });
            TerrainValueType::F64 {
                value_or_index: ValueOrIndex::Index(index),
            }
        }
    }
}
