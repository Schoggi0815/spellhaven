use bevy::prelude::*;
use egui_node_editor::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Default, Resource, Serialize, Deserialize)]
pub struct TerrainGraphState {
    pub preview_node: Option<NodeId>,
}
