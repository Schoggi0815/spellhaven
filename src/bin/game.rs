use bevy::{
    light::{CascadeShadowConfigBuilder, SunDisk, light_consts::lux},
    pbr::wireframe::WireframeConfig,
    prelude::*,
};
use plugins::game_plugins::GamePlugins;

#[tokio::main]
async fn main() {
    App::new()
        .add_plugins(GamePlugins)
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

    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: lux::FULL_DAYLIGHT,
        ..default()
    });
}
