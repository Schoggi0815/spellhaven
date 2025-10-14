use bevy::{
    camera::Exposure,
    core_pipeline::tonemapping::Tonemapping,
    pbr::Atmosphere,
    post_process::bloom::Bloom,
    prelude::*,
    render::view::{ColorGrading, ColorGradingGlobal},
};
use bevy_egui::PrimaryEguiContext;
use bevy_hookup_core::{
    owner_component::Owner, shared::Shared, sync_entity::SyncEntityOwner,
};
use bevy_panorbit_camera::PanOrbitCamera;
use physics::{
    collider::Collider, physics_object::DynamicPhysicsObject,
    physics_position::PhysicsPosition,
};

use serde::{Deserialize, Serialize};
use world_generation::{
    chunk_loading::chunk_loader::ChunkLoader, world_ready::WorldReady,
};

use crate::player_state::PlayerState;

#[derive(Component)]
pub struct Player {
    pub fly: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlayerPosition {
    pub position: Vec3,
    pub velocity: Vec3,
    pub rotation: Quat,
}

#[derive(Component, Default)]
pub struct PlayerSmoothing {
    pub lerp_time: f32,
    pub start_pos: Vec3,
    pub end_pos: Vec3,
}

#[derive(Component)]
pub(super) struct PlayerBody;

#[derive(Component)]
pub struct PlayerCamera;

pub(super) fn spawn_player(
    _: On<WorldReady>,
    mut commands: Commands,
    mut player_state: ResMut<NextState<PlayerState>>,
    mut ray_cast: MeshRayCast,
) -> Result {
    player_state.set(PlayerState::Spawend);

    let ray = Ray3d::new(Vec3::Y * 5000., Dir3::NEG_Y);
    let Some((_, hit)) = ray_cast
        .cast_ray(
            ray,
            &MeshRayCastSettings {
                visibility: RayCastVisibility::Any,
                ..Default::default()
            },
        )
        .first()
    else {
        return Err("Could not spawn player!".into());
    };

    let spawn_point = hit.point + Vec3::Y;

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
        ChunkLoader::default(),
        Name::new("Player"),
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            ..Default::default()
        },
        PrimaryEguiContext,
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

    Ok(())
}

pub(super) fn spawn_player_body(
    players_without_body: Query<
        Entity,
        (With<Shared<PlayerPosition>>, Without<PlayerBody>),
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for player_entity in players_without_body {
        commands
            .entity(player_entity)
            .insert((
                PlayerBody,
                PlayerSmoothing::default(),
                Name::new("PlayerBody"),
                Mesh3d::default(),
            ))
            .with_children(|commands| {
                commands.spawn((
                    SceneRoot(asset_server.load("player.gltf#Scene0")),
                    Transform::from_xyz(0., 0.15, 0.),
                    Name::new("PlayerHead"),
                ));
                commands.spawn((
                    Mesh3d(meshes.add(Mesh::from(Capsule3d {
                        radius: 0.4,
                        half_length: 0.3,
                        ..default()
                    }))),
                    MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
                    Transform::from_xyz(0., -0.35, 0.),
                    Name::new("PlayerTorso"),
                ));
            });
    }
}
