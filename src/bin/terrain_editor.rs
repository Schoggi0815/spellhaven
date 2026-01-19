use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use debug_tools::terrain_node_editor::{
    terrain_graph_resource::TerrainGraphResource,
    terrain_graph_state::TerrainGraphState,
};
use egui::{
    Color32, ColorImage, DragValue, ImageSource, TextureHandle, TextureOptions,
    load::SizedTexture,
};
use egui_node_editor::NodeId;
use itertools::Itertools;
use rand::{SeedableRng, rngs::StdRng};
use rayon::prelude::*;

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
        .insert_resource(PreviewTexture {
            zoom: 1.,
            amplitude: 1.,
            ..Default::default()
        })
        .init_resource::<TerrainGraphState>()
        .run();
}

#[derive(Resource, Default, Clone, PartialEq)]
struct PreviewTexture {
    handle: Option<TextureHandle>,
    origin_node_id: Option<NodeId>,
    zoom: f64,
    amplitude: f64,
    x_offset: f64,
    y_offset: f64,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn render_terrain_editor(
    mut contexts: EguiContexts,
    mut terrain_graph: ResMut<TerrainGraphResource>,
    mut preview_texture: ResMut<PreviewTexture>,
    mut terrain_graph_state: ResMut<TerrainGraphState>,
    mut previous_preview: Local<PreviewTexture>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let mut preview_texture_handle = match &preview_texture.handle {
        None => {
            let texture = ctx.load_texture(
                "Noise Preview",
                ColorImage::example(),
                TextureOptions::NEAREST,
            );
            preview_texture.handle = Some(texture.clone());
            texture
        }
        Some(texture) => texture.clone(),
    };

    if *previous_preview != *preview_texture
        && let Some(node_id) = preview_texture.origin_node_id
    {
        preview_texture.origin_node_id = terrain_graph_state.preview_node;

        if let Some((_, node)) = terrain_graph
            .graph
            .nodes
            .iter()
            .find(|(id, _)| *id == node_id)
        {
            const IMAGE_SIZE: usize = 256;

            let noise = terrain_graph.get_terrain_noise(node);

            let noise_fn = noise.get_noise_fn(&mut StdRng::seed_from_u64(0));

            let mut colors = Vec::new();

            (0..IMAGE_SIZE)
                .cartesian_product(0..IMAGE_SIZE)
                .collect_vec()
                .into_par_iter()
                .map(|(x, y)| {
                    Color32::from_gray(
                        (noise_fn.get([
                            (x as f64 + preview_texture.x_offset)
                                * preview_texture.zoom,
                            (y as f64 + preview_texture.y_offset)
                                * preview_texture.zoom,
                        ]) * 255.
                            / preview_texture.amplitude)
                            as u8,
                    )
                })
                .collect_into_vec(&mut colors);

            let image = ColorImage::new([IMAGE_SIZE, IMAGE_SIZE], colors);

            preview_texture_handle.set(image, TextureOptions::NEAREST);
        }
    }

    *previous_preview = preview_texture.clone();

    preview_texture.origin_node_id = terrain_graph_state.preview_node;

    egui::SidePanel::right("right")
        .resizable(false)
        .frame(egui::Frame::new().fill(egui::Color32::from_gray(25)))
        .show(ctx, |ui| {
            ui.image(ImageSource::Texture(SizedTexture::from_handle(
                &preview_texture_handle,
            )));

            let zoom_speed = preview_texture.zoom * 0.1;
            ui.add(
                DragValue::new(&mut preview_texture.zoom)
                    .range(0.0..=f64::MAX)
                    .speed(zoom_speed),
            );
            let amp_speed = preview_texture.amplitude * 0.1;
            ui.add(
                DragValue::new(&mut preview_texture.amplitude)
                    .range(0.0..=f64::MAX)
                    .speed(amp_speed),
            );
            ui.add(DragValue::new(&mut preview_texture.x_offset));
            ui.add(DragValue::new(&mut preview_texture.y_offset));
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
        .show(ctx, |ui| terrain_graph.draw(ui, &mut terrain_graph_state));

    Ok(())
}
