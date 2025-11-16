use bevy::prelude::*;
use bevy_hookup_core::from_session::FromSession;

use crate::network_physics_object::NetworkPhysicsObject;

const BUFFER_SIZE: usize = 4;

#[derive(Component, Default, Debug, Reflect)]
#[require(Transform)]
pub struct NetworkPhysicsBuffer {
    pub buffer: [Vec3; BUFFER_SIZE + 1],
    pub current_network_index: u64,
    pub latest_velocity: Vec3,
}

impl NetworkPhysicsBuffer {
    fn next(&mut self, next_pos: Vec3) {
        for i in 0..BUFFER_SIZE {
            self.buffer[i] = self.buffer[i + 1];
        }

        self.buffer[BUFFER_SIZE] = next_pos;
    }

    fn actual_position(&self, lerp: f32) -> Vec3 {
        self.buffer[0].lerp(self.buffer[1], lerp)
    }
}

pub fn add_buffer(
    shared_physics: Query<
        (Entity, &NetworkPhysicsObject),
        (Without<NetworkPhysicsBuffer>, With<FromSession>),
    >,
    mut commands: Commands,
) {
    for (entity, network_object) in shared_physics {
        commands.entity(entity).insert(NetworkPhysicsBuffer {
            buffer: [network_object.position; BUFFER_SIZE + 1],
            current_network_index: network_object.update_index,
            latest_velocity: network_object.velocity,
        });
    }
}

pub fn update_network_physics_buffer(
    smoothings: Query<(&mut NetworkPhysicsBuffer, Ref<NetworkPhysicsObject>)>,
    time: Res<Time>,
) {
    for (mut network_buffer, network_object) in smoothings {
        if network_object.is_changed()
            && network_object.update_index
                > network_buffer.current_network_index
        {
            network_buffer.current_network_index = network_object.update_index;
            network_buffer.next(network_object.position);
            network_buffer.latest_velocity = network_object.velocity;
            continue;
        }

        let next_pos = network_buffer.buffer[BUFFER_SIZE]
            + network_buffer.latest_velocity * time.delta_secs();
        network_buffer.next(next_pos);

        if network_object.is_changed()
            && network_buffer.current_network_index >= BUFFER_SIZE as u64
            && network_object.update_index
                >= network_buffer.current_network_index - BUFFER_SIZE as u64
            && network_object.update_index
                < network_buffer.current_network_index
        {
            let buffer_index = BUFFER_SIZE as u64 + network_object.update_index
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
