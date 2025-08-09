use bevy::{
    pbr::{
        ExtendedMaterial,
        light_consts::lux,
        wireframe::{WireframeConfig, WireframePlugin},
    },
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use spellhaven::{
    animation::animation_plugin::SpellhavenAnimationPlugin,
    debug_tools::debug_plugin::SpellhavenDebugPlugin,
    physics::physics_plugin::PhysicsPlugin,
    player::player_plugin::PlayerPlugin,
    ui::game_ui_plugin::GameUiPlugin,
    world_generation::{
        terrain_material::TerrainMaterial,
        world_generation_plugin::WorldGenerationPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Spellhaven".into(),
                            // present_mode: PresentMode::Immediate,
                            ..default()
                        }),
                        ..default()
                    })
                    .set(ImagePlugin::default_nearest()),
                PanOrbitCameraPlugin,
                WorldGenerationPlugin,
                PhysicsPlugin,
                PlayerPlugin,
                WireframePlugin { ..default() },
                SpellhavenAnimationPlugin,
                EguiPlugin::default(),
                WorldInspectorPlugin::new(),
                GameUiPlugin,
                SpellhavenDebugPlugin,
                MaterialPlugin::<
                    ExtendedMaterial<StandardMaterial, TerrainMaterial>,
                >::default(),
            ),
        )
        .add_systems(Startup, setup)
        .insert_resource(WireframeConfig {
            global: false,
            default_color: Color::srgb(1., 0., 0.),
        })
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 3.),
            ..default()
        },
        Name::new("Light"),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: lux::FULL_DAYLIGHT,
        ..default()
    });
}
