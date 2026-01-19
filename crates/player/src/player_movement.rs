use std::f32::consts::PI;

use bevy::prelude::*;
use physics::{
    physics_object::DynamicPhysicsObject, physics_velocity::PhysicsVelocity,
};

use crate::{
    camera::player_camera::PlayerCamera,
    player_component::{Player, PlayerRotation},
    player_inputs::PlayerInputs,
};

pub(super) fn movement(
    player_input: Res<PlayerInputs>,
    mut players: Query<(
        &mut Player,
        &mut PhysicsVelocity,
        &mut DynamicPhysicsObject,
        &mut PlayerRotation,
    )>,
    player_camera: Query<&PlayerCamera>,
    time: Res<Time>,
) {
    for (
        mut player,
        mut physics_velocity,
        mut physics_object,
        mut player_rotation,
    ) in &mut players
    {
        let mut move_direction = Vec3::ZERO;

        let grounded = physics_object.touching_sides.y < 0;

        if player_input.fly {
            player.fly = !player.fly;
        }

        if grounded || player.fly {
            physics_velocity.y = 0.;
        }

        // Directional movement
        if player_input.forward {
            move_direction.z -= 1.;
        }
        if player_input.left {
            move_direction.x -= 1.;
        }
        if player_input.backwards {
            move_direction.z += 1.;
        }
        if player_input.right {
            move_direction.x += 1.;
        }

        if player.fly && player_input.up {
            move_direction.y += 1.;
        }
        if player.fly && player_input.down {
            move_direction.y -= 1.;
        }

        let mut movement_speed = if player_input.sprint { 15. } else { 7.5 };

        if player.fly {
            movement_speed *= 10.;
        }

        if let Ok(player_camera) = player_camera.single() {
            // Rotate vector to camera
            let rotation = Quat::from_rotation_y(player_camera.yaw);
            move_direction = rotation
                .mul_vec3(move_direction.normalize_or_zero() * movement_speed);

            if move_direction.xz() != Vec2::ZERO {
                player_rotation.0 = Quat::from_rotation_y(
                    -move_direction.xz().to_angle() - PI * 0.5,
                );
            }
        }

        // Jump if space pressed and the player is close enough to the ground
        if !player.fly && grounded && player_input.jump {
            physics_velocity.y += 10.;
            physics_object.touching_sides.y = 0;
        }

        physics_velocity.x = move_direction.x;
        physics_velocity.z = move_direction.z;
        if !player.fly {
            if grounded {
                physics_velocity.y -= 100. * time.delta_secs();
            } else {
                physics_velocity.y -= (20. * time.delta_secs()).max(-160.);
            }
        } else {
            physics_velocity.y = move_direction.y;
        }
    }
}
