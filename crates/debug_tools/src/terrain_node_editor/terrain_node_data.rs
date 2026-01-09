use egui::{
    ComboBox, Image, ImageSource, TextureHandle, Vec2, load::SizedTexture,
};
use egui_node_editor::{Graph, NodeDataTrait, NodeId, NodeResponse};
use serde::{Deserialize, Serialize};

use crate::terrain_node_editor::{
    noise_output_type::ALL_NOISE_OUTPUT_TYPES,
    terrain_data_type::TerrainDataType,
    terrain_node_template::TerrainNodeTemplate,
    terrain_response::TerrainResponse, terrain_user_state::TerrainUserState,
    terrain_value_type::TerrainValueType,
};

#[derive(Serialize, Deserialize)]
pub struct TerrainNodeData {
    pub template: TerrainNodeTemplate,
}

impl NodeDataTrait for TerrainNodeData {
    type Response = TerrainResponse;

    type UserState = TerrainUserState;

    type DataType = TerrainDataType;

    type ValueType = TerrainValueType;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>>
    where
        Self::Response: egui_node_editor::UserResponseTrait,
    {
        match self.template {
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
            TerrainNodeTemplate::Preview(_cached_texture) => {
                // let values = Vec::new();

                // for x in 0..64 {
                //     for y in 0..64 {
                //         values.push(x * 4);
                //     }
                // }

                // let image = egui::ColorImage::from_gray([64, 64], &values);

                ui.image(ImageSource::Texture(SizedTexture::new(
                    _cached_texture,
                    Vec2::new(128., 128.),
                )));

                vec![]
            }
            _ => {
                vec![]
            }
        }
    }
}
