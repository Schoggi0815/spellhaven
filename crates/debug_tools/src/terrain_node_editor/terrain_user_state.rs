use egui::TextureHandle;

pub struct TerrainUserState {
    preview_textures: Vec<TextureHandle>,
}

impl Default for TerrainUserState {
    fn default() -> Self {
        Self {
            preview_textures: Default::default(),
        }
    }
}
