use bevy::prelude::*;
use plugins::terrain_editor_plugin::TerrainEditorPlugin;

fn main() {
    App::new().add_plugins(TerrainEditorPlugin).run();
}
