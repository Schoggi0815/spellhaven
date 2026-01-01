use egui::ComboBox;
use egui_node_editor::{Graph, NodeDataTrait, NodeId, NodeResponse};
use serde::{Deserialize, Serialize};

use crate::terrain_node_editor::{
    noise_output_type::ALL_NOISE_OUTPUT_TYPES,
    terrain_data_type::TerrainDataType,
    terrain_node_template::TerrainNodeTemplate,
    terrain_response::TerrainResponse, terrain_value_type::TerrainValueType,
};

#[derive(Serialize, Deserialize)]
pub struct TerrainNodeData {
    pub template: TerrainNodeTemplate,
}

impl NodeDataTrait for TerrainNodeData {
    type Response = TerrainResponse;

    type UserState = ();

    type DataType = TerrainDataType;

    type ValueType = TerrainValueType;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>>
    where
        Self::Response: egui_node_editor::UserResponseTrait,
    {
        let Some((_, node)) = graph.nodes.iter().find(|n| n.0 == node_id)
        else {
            return vec![];
        };

        match node.user_data.template {
            TerrainNodeTemplate::Output(output) => {
                let mut new_value = output.clone();

                ComboBox::from_label("Output type")
                    .selected_text(output.get_text())
                    .show_ui(ui, |ui| {
                        for noise_output_type in ALL_NOISE_OUTPUT_TYPES {
                            noise_output_type
                                .render_selectable_value(ui, &mut new_value);
                        }
                    });

                if new_value != output {
                    vec![NodeResponse::User(TerrainResponse::UpdateOutputType(
                        node_id, new_value,
                    ))]
                } else {
                    vec![]
                }
            }
            _ => {
                vec![]
            }
        }
    }
}
