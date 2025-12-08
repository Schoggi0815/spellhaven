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
    from_session::FromSession,
    receive_component_systems::ReceiveComponentSystems,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use debug_tools::{
    debug_plugin::SpellhavenDebugPlugin, physics_debug::PhysicsDebugResource,
};
use physics::{
    collider::Collider,
    network_physics_object::NetworkPhysicsObject,
    physics_object::{DynamicPhysicsObject, StaticPhysicsObject},
    physics_plugin::PhysicsPlugin,
    physics_position::PhysicsPosition,
    physics_systems::PhysicsSystems,
    physics_velocity::PhysicsVelocity,
};
use player::{
    player_component::{Player, PlayerCamera, PlayerRotation},
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
            SpellhavenDebugPlugin,
        ))
        .init_resource::<PhysicsDebugResource>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            mock_network.after(PhysicsSystems).in_set(
                ReceiveComponentSystems::<NetworkPhysicsObject>::default(),
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_point = Vec3::new(0., 2., 0.);

    commands.spawn((
        StaticPhysicsObject,
        Collider::aabb(Vec3::new(100., 1., 100.), Vec3::ZERO),
        Transform::from_translation(Vec3::ZERO),
        Mesh3d(meshes.add(Cuboid::new(100., 1., 100.))),
        MeshMaterial3d(
            materials.add(StandardMaterial::from_color(Color::WHITE)),
        ),
    ));

    // Player
    commands.spawn((
        DynamicPhysicsObject {
            step_height: 0.6,
            ..Default::default()
        },
        PhysicsPosition(spawn_point),
        Transform::from_translation(spawn_point),
        Collider::aabb(Vec3::new(0.8, 1.8, 0.8), Vec3::ZERO),
        Player { fly: false },
        PlayerRotation::default(),
        Name::new("Player"),
    ));

    commands.spawn((
        NetworkPhysicsObject::default(),
        FromSession::default(),
        PlayerRotation::default(),
        Mesh3d(meshes.add(Cuboid::new(1., 2., 1.))),
        MeshMaterial3d(
            materials.add(StandardMaterial::from_color(Color::WHITE)),
        ),
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

fn mock_network(
    player: Single<
        (&PhysicsPosition, &PhysicsVelocity, &PlayerRotation),
        Without<NetworkPhysicsObject>,
    >,
    mut mock: Single<(&mut NetworkPhysicsObject, &mut PlayerRotation)>,
) {
    mock.1.0 = player.2.0;

    if mock.0.position == **player.0 {
        return;
    }

    mock.0.position = **player.0;
    mock.0.update_index += 1;
}
