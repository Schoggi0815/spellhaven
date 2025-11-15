use bevy::prelude::*;
use bevy_hookup_core::shared::Shared;

use crate::network_physics_object::NetworkPhysicsObject;

#[derive(Component, Default, Debug)]
pub struct NetworkPhysicsSmoothing {
    start_position: Vec3,
    end_position: Vec3,
    lerp_amount: f32,
    index: u64,
}

pub fn add_smoothing(
    shared_physics: Query<
        (Entity, &Shared<NetworkPhysicsObject>),
        (Without<NetworkPhysicsSmoothing>,),
    >,
    mut commands: Commands,
) {
    for (entity, network_object) in shared_physics {
        commands.entity(entity).insert(NetworkPhysicsSmoothing {
            start_position: network_object.position,
            end_position: network_object.position,
            lerp_amount: 0.0,
            index: network_object.update_index,
        });
    }
}

pub fn update_network_physics_smoothing(
    smoothings: Query<
        (&mut NetworkPhysicsSmoothing, &Shared<NetworkPhysicsObject>),
        Changed<Shared<NetworkPhysicsObject>>,
    >,
) {
    for (mut network_smoothing, network_object) in smoothings {
        if network_object.update_index < network_smoothing.index {
            warn!("Order of updates backwards!");
            continue;
        }
        let index_difference =
            network_object.update_index - network_smoothing.index;
        network_smoothing.start_position = network_smoothing
            .start_position
            .lerp(network_smoothing.end_position, index_difference as f32);
        network_smoothing.end_position = network_object.position;
        network_smoothing.lerp_amount = 0.0;
        network_smoothing.index = network_object.update_index;
    }
}

pub fn move_network_physics_smoothed(
    smoothings: Query<(&mut NetworkPhysicsSmoothing, &mut Transform)>,
    time_fixed: Res<Time<Fixed>>,
) {
    for (mut network_smoothing, mut transform) in smoothings {
        let mut overstep_fraction = time_fixed.overstep_fraction();
        while overstep_fraction < network_smoothing.lerp_amount {
            overstep_fraction += 1.;
        }
        network_smoothing.lerp_amount = overstep_fraction;
        transform.translation = network_smoothing
            .start_position
            .as_dvec3()
            .lerp(
                network_smoothing.end_position.as_dvec3(),
                network_smoothing.lerp_amount as f64,
            )
            .as_vec3();
    }
}
