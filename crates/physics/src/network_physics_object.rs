use bevy::prelude::*;
use bevy_hookup_core::owner_component::Owner;
use serde::{Deserialize, Serialize};

use crate::{
    physics_position::PhysicsPosition, physics_velocity::PhysicsVelocity,
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct NetworkPhysicsObject {
    pub position: Vec3,
    pub velocity: Vec3,
    pub update_index: u64
}

pub fn update_network_physics(
    player: Single<(
        &mut Owner<NetworkPhysicsObject>,
        &PhysicsPosition,
        &PhysicsVelocity,
    )>,
) {
    let (mut network_physics, physics_position, physics_velocity) =
        player.into_inner();

    if network_physics.position == **physics_position
        && network_physics.velocity == **physics_velocity
    {
        return;
    }

    network_physics.position = **physics_position;
    network_physics.velocity = **physics_velocity;
    network_physics.update_index += 1;
}
