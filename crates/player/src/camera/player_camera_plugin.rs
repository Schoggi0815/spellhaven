use core::f32;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use utils::math::lerp_decay;
use world_generation::chunk_generation::terrain_mesh::TerrainMesh;

use crate::camera::{
    player_camera::PlayerCamera, player_camera_lerp::PlayerCameraLerp,
    player_camera_target::PlayerCameraTarget,
};

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_camera_transform.after(lerp_camera),
                lerp_camera.after(camera_movement),
                camera_movement,
            ),
        );
    }
}

fn update_camera_transform(
    camera: Single<(&mut Transform, &PlayerCamera)>,
    terrain_meshes: Query<(), With<TerrainMesh>>,
    mut ray_cast: MeshRayCast,
) {
    let (mut camera_transform, camera_component) = camera.into_inner();

    let target = camera_component.target_pos + camera_component.target_offset;

    let mut camera_position = Vec3::Z * camera_component.distance;
    camera_position =
        Quat::from_rotation_x(camera_component.pitch) * camera_position;
    camera_position =
        Quat::from_rotation_y(camera_component.yaw) * camera_position;

    let ray = ray_cast
        .cast_ray(
            Ray3d::new(target, Dir3::new(camera_position).unwrap()),
            &MeshRayCastSettings::default()
                .always_early_exit()
                .with_visibility(RayCastVisibility::Visible)
                .with_filter(&|entity| terrain_meshes.contains(entity)),
        )
        .first();

    if let Some((_, ray)) = ray
        && ray.distance < camera_component.distance + 1.
    {
        let dot = camera_position.normalize().dot(ray.normal.normalize());
        let offset = (1. - dot) * 0.5;
        camera_position *=
            (ray.distance - offset).max(0.01) / camera_component.distance;
    }

    camera_transform.translation = camera_position + target;
    *camera_transform = camera_transform.looking_at(target, Vec3::Y);
}

fn camera_movement(
    mut camera: Single<&mut PlayerCameraLerp>,
    mut wheel_events: MessageReader<MouseWheel>,
    mut motion_events: MessageReader<MouseMotion>,
) {
    for wheel_event in wheel_events.read() {
        camera.distance -= wheel_event.y;
        camera.distance = camera.distance.max(3.);
    }

    for motion_event in motion_events.read() {
        camera.yaw += motion_event.delta.x * -0.005;
        camera.yaw = (camera.yaw + f32::consts::PI)
            .rem_euclid(f32::consts::PI * 2.)
            - f32::consts::PI;

        camera.pitch += motion_event.delta.y * -0.005;
        let pitch_limit = f32::consts::FRAC_PI_2 * 0.99;
        camera.pitch = camera.pitch.clamp(-pitch_limit, pitch_limit);
    }
}

fn lerp_camera(
    camera: Single<(&mut PlayerCamera, &PlayerCameraLerp)>,
    target: Single<&Transform, With<PlayerCameraTarget>>,
    time: Res<Time>,
) {
    let (mut camera, camera_lerp) = camera.into_inner();

    camera.target_pos = lerp_decay(
        camera.target_pos,
        target.translation,
        16.,
        time.delta_secs(),
    );

    camera.distance = lerp_decay(
        camera.distance,
        camera_lerp.distance,
        16.,
        time.delta_secs(),
    );

    let cutoff = f32::consts::FRAC_PI_2;

    let yaw = if camera.yaw < -cutoff && camera_lerp.yaw > cutoff {
        camera_lerp.yaw - f32::consts::PI * 2.
    } else if camera.yaw > cutoff && camera_lerp.yaw < -cutoff {
        camera_lerp.yaw + f32::consts::PI * 2.
    } else {
        camera_lerp.yaw
    };
    camera.yaw = lerp_decay(camera.yaw, yaw, 16., time.delta_secs());
    camera.yaw = (camera.yaw + f32::consts::PI)
        .rem_euclid(f32::consts::PI * 2.)
        - f32::consts::PI;

    camera.pitch =
        lerp_decay(camera.pitch, camera_lerp.pitch, 16., time.delta_secs());
}
