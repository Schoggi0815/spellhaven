use bevy::prelude::*;
use bevy_hookup_core::shared::Shared;

use crate::network_physics_object::NetworkPhysicsObject;

#[derive(Component, Default, Debug)]
pub struct NetworkPhysicsBuffer {
    pub buffer: [Vec3; 4],
    pub current_network_index: u64,
    pub latest_velocity: Vec3,
}

impl NetworkPhysicsBuffer {
    fn next(&mut self, next_pos: Vec3) {
        self.buffer =
            [self.buffer[1], self.buffer[2], self.buffer[3], next_pos];
        self.current_network_index += 1;
    }

    fn actual_position(&self, lerp: f32) -> Vec3 {
        self.buffer[0].lerp(self.buffer[1], lerp)
    }
}

pub fn add_buffer(
    shared_physics: Query<
        (Entity, &Shared<NetworkPhysicsObject>),
        Without<NetworkPhysicsBuffer>,
    >,
    mut commands: Commands,
) {
    for (entity, network_object) in shared_physics {
        commands.entity(entity).insert(NetworkPhysicsBuffer {
            buffer: [network_object.position; 4],
            current_network_index: network_object.update_index,
            latest_velocity: network_object.velocity,
        });
    }
}

pub fn update_network_physics_buffer(
    smoothings: Query<(
        &mut NetworkPhysicsBuffer,
        Ref<Shared<NetworkPhysicsObject>>,
    )>,
    time: Res<Time>,
) {
    for (mut network_buffer, network_object) in smoothings {
        if network_object.update_index
            == network_buffer.current_network_index + 1
        {
            network_buffer.next(network_object.position);
            network_buffer.latest_velocity = network_object.velocity;
            continue;
        }

        let next_pos = network_buffer.buffer[3]
            + network_buffer.latest_velocity * time.delta_secs();
        network_buffer.next(next_pos);

        if network_object.is_changed()
            && network_buffer.current_network_index >= 3
            && network_object.update_index
                >= network_buffer.current_network_index - 3
            && network_object.update_index
                < network_buffer.current_network_index
        {
            let buffer_index = 3 + network_object.update_index
                - network_buffer.current_network_index;

            network_buffer.buffer[buffer_index as usize] =
                network_object.position;
        }
    }
}

pub fn move_network_physics_buffered(
    smoothings: Query<(&NetworkPhysicsBuffer, &mut Transform)>,
    time_fixed: Res<Time<Fixed>>,
) {
    for (network_buffer, mut transform) in smoothings {
        transform.translation =
            network_buffer.actual_position(time_fixed.overstep_fraction());
    }
}
