use avian3d::prelude::{LinearVelocity, ShapeHits};
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::player::player_component::{Player, PlayerBody, PlayerCamera};

pub(super) fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut LinearVelocity, &ShapeHits), With<Player>>,
    player_camera: Query<&PanOrbitCamera, With<PlayerCamera>>,
) {
    for (mut linear_velocity, ground_hits) in &mut players {
        let mut move_direction = Vec3::ZERO;

        if !ground_hits.is_empty() {
            linear_velocity.y = 0.0;
        } else {
            linear_velocity.y -= 0.4;
        }

        // Directional movement
        if keyboard_input.pressed(KeyCode::KeyW)
            || keyboard_input.pressed(KeyCode::ArrowUp)
        {
            move_direction.z -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyA)
            || keyboard_input.pressed(KeyCode::ArrowLeft)
        {
            move_direction.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyS)
            || keyboard_input.pressed(KeyCode::ArrowDown)
        {
            move_direction.z += 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyD)
            || keyboard_input.pressed(KeyCode::ArrowRight)
        {
            move_direction.x += 1.;
        }

        let movement_speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
            2.
        } else {
            1.
        };

        if let Ok(player_camera) = player_camera.single() {
            // Rotate vector to camera
            let rotation =
                Quat::from_rotation_y(player_camera.yaw.unwrap_or(0.));
            move_direction = rotation
                .mul_vec3(move_direction.normalize_or_zero() * movement_speed);
        }

        // Jump if space pressed and the player is close enough to the ground
        if keyboard_input.pressed(KeyCode::Space) && !ground_hits.is_empty() {
            move_direction.y = 10.;
        }

        linear_velocity.0 += move_direction;

        // Slow player down
        linear_velocity.x *= 0.8;
        linear_velocity.y *= 0.98;
        linear_velocity.z *= 0.8;
    }
}

pub(super) fn move_body(
    player: Query<&Transform, (With<Player>, Without<PlayerBody>)>,
    mut player_body: Query<&mut Transform, (With<PlayerBody>, Without<Player>)>,
) {
    let (Ok(player), Ok(mut player_body)) =
        (player.single(), player_body.single_mut())
    else {
        return;
    };

    let difference = player.translation - player_body.translation;
    player_body.translation += difference * 0.25;
    player_body.rotation = player_body.rotation.lerp(player.rotation, 0.25);
}
