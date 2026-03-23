use bevy::{
    app::PluginGroupBuilder,
    pbr::{ExtendedMaterial, wireframe::WireframePlugin},
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_steamworks::SteamworksPlugin;
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

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(SteamworksPlugin::init_app(4251410).unwrap())
            .add_group(
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
            )
            .add(PanOrbitCameraPlugin)
            .add(WorldGenerationPlugin)
            .add(PhysicsPlugin)
            .add(PlayerPlugin)
            .add(WireframePlugin { ..default() })
            .add(EguiPlugin::default())
            .add(WorldInspectorPlugin::new())
            .add(GameUiPlugin)
            .add(MainMenuPlugin)
            .add(SpellhavenDebugPlugin)
            .add(NetworkingPlugin)
            .add(MaterialPlugin::<
                ExtendedMaterial<StandardMaterial, TerrainMaterial>,
            >::default())
    }
}
