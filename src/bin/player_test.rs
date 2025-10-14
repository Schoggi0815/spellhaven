use bevy::{
    camera::Exposure,
    core_pipeline::tonemapping::Tonemapping,
    light::{CascadeShadowConfigBuilder, SunDisk, light_consts::lux},
    pbr::Atmosphere,
    post_process::bloom::Bloom,
    prelude::*,
    render::view::{ColorGrading, ColorGradingGlobal},
};
use bevy_egui::EguiPlugin;
use bevy_hookup_core::{
    hook_session::SessionMessenger, owner_component::Owner,
    sync_entity::SyncEntityOwner,
};
use bevy_hookup_messenger_self::self_session::SelfSession;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use debug_tools::{
    debug_plugin::SpellhavenDebugPlugin, physics_debug::PhysicsDebugResource,
};
use networking::{networking_plugin::NetworkingPlugin, sendables::Sendables};
use physics::{
    collider::Collider,
    physics_object::{DynamicPhysicsObject, StaticPhysicsObject},
    physics_plugin::PhysicsPlugin,
    physics_position::PhysicsPosition,
};
use player::{
    player_component::{Player, PlayerCamera, PlayerPosition},
    player_plugin::PlayerPlugin,
};

fn main() {
    App::new()
        .add_plugins((
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
            PhysicsPlugin,
            PlayerPlugin,
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            NetworkingPlugin,
            SpellhavenDebugPlugin,
        ))
        .init_resource::<PhysicsDebugResource>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let spawn_point = Vec3::new(0., 2., 0.);

    commands.spawn((
        StaticPhysicsObject,
        Collider::aabb(Vec3::new(100., 1., 100.), Vec3::ZERO),
        Transform::from_translation(Vec3::ZERO),
    ));

    // Player
    commands.spawn((
        DynamicPhysicsObject {
            step_height: 0.6,
            ..Default::default()
        },
        PhysicsPosition {
            position: spawn_point,
            ..Default::default()
        },
        Transform::from_translation(spawn_point),
        Collider::aabb(Vec3::new(0.8, 1.8, 0.8), Vec3::ZERO),
        SyncEntityOwner::new(),
        Player { fly: false },
        Owner::new(PlayerPosition {
            position: spawn_point,
            velocity: Vec3::ZERO,
            rotation: Quat::default(),
        }),
        Name::new("Player"),
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            ..Default::default()
        },
        ColorGrading {
            global: ColorGradingGlobal {
                post_saturation: 1.2,
                ..Default::default()
            },
            ..Default::default()
        },
        Msaa::Sample2,
        // Smaa {
        //     preset: SmaaPreset::Ultra,
        //     ..Default::default()
        // },
        Transform::from_xyz(-4.0, 6.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            far: 2f32.powi(20),
            ..default()
        }),
        Exposure::SUNLIGHT,
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        PanOrbitCamera::default(),
        Atmosphere::EARTH,
        PlayerCamera,
        Name::new("PlayerCamera"),
    ));

    commands.spawn(SelfSession::<Sendables>::new().to_session());

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
