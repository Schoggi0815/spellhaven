use bevy::{
    light::{CascadeShadowConfigBuilder, SunDisk, light_consts::lux},
    pbr::{
        ExtendedMaterial,
        wireframe::{WireframeConfig, WireframePlugin},
    },
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use debug_tools::debug_plugin::SpellhavenDebugPlugin;
use main_menu::main_menu_plugin::MainMenuPlugin;
use networking::networking_plugin::NetworkingPlugin;
use physics::physics_plugin::PhysicsPlugin;
use player::player_plugin::PlayerPlugin;
use ui::game_ui_plugin::GameUiPlugin;
use world_generation::{
    terrain_material::TerrainMaterial,
    world_generation_plugin::WorldGenerationPlugin,
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
                EguiPlugin::default(),
                WorldInspectorPlugin::new(),
                GameUiPlugin,
                MainMenuPlugin,
                SpellhavenDebugPlugin,
                NetworkingPlugin,
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
        SunDisk::EARTH,
        CascadeShadowConfigBuilder {
            // num_cascades: 10,
            ..Default::default()
        }
        .build(),
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
