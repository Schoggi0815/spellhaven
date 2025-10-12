use bevy::{
    pbr::MaterialExtension, prelude::*, render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
pub struct TerrainMaterial {
    #[uniform(100)]
    pub chunk_position: Vec3,
    #[uniform(100)]
    pub lod_multiplier: u32,
}

impl MaterialExtension for TerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_material.wgsl".into()
    }
}
