use bevy::prelude::*;
use bevy_hookup_core::shared::Shared;

use crate::network_physics_object::NetworkPhysicsObject;

#[derive(Component, Default)]
pub struct NetworkPhysicsSmoothing {
    lerp_time: f32,
    start_pos: Vec3,
    end_pos: Vec3,
}

pub fn add_smoothing(
    shared_physics: Query<
        Entity,
        (
            With<Shared<NetworkPhysicsObject>>,
            Without<NetworkPhysicsSmoothing>,
        ),
    >,
    mut commands: Commands,
) {
    for entity in shared_physics {
        commands
            .entity(entity)
            .insert(NetworkPhysicsSmoothing::default());
    }
}

pub fn update_network_physics_smoothing(
    smoothings: Query<
        (
            &mut NetworkPhysicsSmoothing,
            &Shared<NetworkPhysicsObject>,
            &Transform,
        ),
        Changed<Shared<NetworkPhysicsObject>>,
    >,
) {
    for (mut network_smoothing, network_object, transform) in smoothings {
        network_smoothing.lerp_time = 0.;
        network_smoothing.start_pos = transform.translation;
        network_smoothing.end_pos =
            network_object.position + network_object.velocity * 0.1;
    }
}

pub fn move_network_physics_smoothed(
    smoothings: Query<(&mut NetworkPhysicsSmoothing, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut network_smoothing, mut transform) in smoothings {
        network_smoothing.lerp_time += time.delta_secs() * 10.;
        let new_pos = network_smoothing.start_pos.lerp(
            network_smoothing.end_pos,
            network_smoothing.lerp_time.min(1.),
        );
        transform.translation = new_pos;
    }
}
