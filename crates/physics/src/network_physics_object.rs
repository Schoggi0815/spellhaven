use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::physics_position::PhysicsPosition;

#[derive(Clone, Reflect, Debug, Serialize, Deserialize, Default, Component)]
pub struct NetworkPhysicsObject {
    pub position: Vec3,
    pub update_index: u64,
}

pub fn update_network_physics(
    player: Single<(&mut NetworkPhysicsObject, &PhysicsPosition)>,
    mut last_changed: Local<bool>,
) {
    let (mut network_physics, physics_position) = player.into_inner();

    if network_physics.position == **physics_position && !*last_changed {
        return;
    }

    *last_changed = network_physics.position != **physics_position;

    network_physics.position = **physics_position;
    network_physics.update_index += 1;
}
