use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use debug_tools::terrain_node_editor::terrain_graph_resource::TerrainGraphResource;
use egui::{
    ColorImage, ImageSource, TextureHandle, TextureOptions, load::SizedTexture,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Spellhaven - Terrain Editor".into(),
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin::default(),
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(EguiPrimaryContextPass, render_terrain_editor)
        .init_resource::<TerrainGraphResource>()
        .init_resource::<PreviewTexture>()
        .run();
}

#[derive(Resource, Default)]
struct PreviewTexture(Option<TextureHandle>);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn render_terrain_editor(
    mut contexts: EguiContexts,
    mut terrain_graph: ResMut<TerrainGraphResource>,
    mut preview_texture: ResMut<PreviewTexture>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let preview_texture = match &preview_texture.0 {
        None => {
            let texture = ctx.load_texture(
                "Noise Preview",
                ColorImage::example(),
                TextureOptions::NEAREST,
            );
            preview_texture.0 = Some(texture.clone());
            texture
        }
        Some(texture) => texture.clone(),
    };

    egui::SidePanel::right("right")
        .resizable(false)
        .frame(egui::Frame::new().fill(egui::Color32::from_gray(25)))
        .show(ctx, |ui| {
            ui.image(ImageSource::Texture(SizedTexture::from_handle(
                &preview_texture,
            )))
        });

    egui::TopBottomPanel::bottom("bottom")
        .frame(
            egui::Frame::new()
                .fill(egui::Color32::from_gray(25))
                .inner_margin(2),
        )
        .show(ctx, |ui| {
            if ui.button("Save").clicked() {
                terrain_graph.save().expect("Could not save!");
            }
        });

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(egui::Color32::from_gray(25)))
        .show(ctx, |ui| terrain_graph.draw(ui));

    Ok(())
}
