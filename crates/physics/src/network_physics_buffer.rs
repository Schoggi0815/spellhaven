use bevy::prelude::*;
use bevy_hookup_core::from_session::FromSession;
use itertools::{FoldWhile, Itertools};

use crate::network_physics_object::NetworkPhysicsObject;

const BUFFER_SIZE: usize = 3;

#[derive(Component, Default, Debug, Reflect)]
#[require(Transform)]
pub struct NetworkPhysicsBuffer {
    pub buffer: [Option<Vec3>; BUFFER_SIZE],
    pub current_network_index: u64,
    pub actual_current: Vec3,
    pub actual_last: Vec3,
}

impl NetworkPhysicsBuffer {
    fn try_pop(&mut self) {
        let Some(first) = self.buffer[0] else {
            info!("Skipping one buffer cycle");
            return;
        };

        self.actual_last = self.actual_current;
        self.actual_current = first;

        for i in 0..BUFFER_SIZE - 1 {
            self.buffer[i] = self.buffer[i + 1];
        }

        self.buffer[BUFFER_SIZE - 1] = None;
        self.current_network_index += 1;
    }

    fn set_position(&mut self, network_index: u64, position: Vec3) {
        let array_index = (network_index - self.current_network_index) as usize;
        info!(
            "network: {}, current_network: {}",
            network_index, self.current_network_index
        );

        if array_index < BUFFER_SIZE {
            self.buffer[array_index] = Some(position);

            if array_index > 0 && self.buffer[array_index - 1].is_none() {
                let (latest_position, latest_index) = self.get_latest();

                for i in (latest_index + 1) as usize..array_index {
                    let lerp_value = (i as f32 - latest_index as f32)
                        / (array_index as f32 - latest_index as f32);

                    self.buffer[i] =
                        Some(latest_position.lerp(position, lerp_value));
                }
            }

            return;
        }

        let shift = array_index - BUFFER_SIZE + 1;
        let array_index = BUFFER_SIZE - 1;
        info!("Shifting up buffer by: {}", shift);

        let (latest_position, latest_index) = self.get_latest();
        let latest_index = latest_index - shift as i32;

        let lerp_max = array_index as f32 - latest_index as f32;

        if shift <= 1 {
            self.actual_last = self.actual_current;
        } else {
            let actual_last_index = shift - 2;

            let actual_last =
                self.buffer.get(actual_last_index).cloned().flatten();

            self.actual_last = if let Some(actual_last) = actual_last {
                actual_last
            } else {
                latest_position.lerp(
                    position,
                    (actual_last_index as f32 - latest_index as f32) / lerp_max,
                )
            };
        }

        let actual_current_index = shift - 1;
        let actual_current =
            self.buffer.get(actual_current_index).cloned().flatten();

        self.actual_current = if let Some(actual_current) = actual_current {
            actual_current
        } else {
            latest_position.lerp(
                position,
                (actual_current_index as f32 - latest_index as f32) / lerp_max,
            )
        };

        for i in 0..BUFFER_SIZE - 1 {
            let before_index = i + shift;
            let new = self.buffer.get(before_index).cloned().flatten();

            self.buffer[i] = if let Some(new) = new {
                Some(new)
            } else {
                Some(latest_position.lerp(
                    position,
                    (before_index as f32 - latest_index as f32) / lerp_max,
                ))
            };
        }

        self.buffer[array_index] = Some(position);
        self.current_network_index += shift as u64;
    }

    fn get_latest(&self) -> (Vec3, i32) {
        self.buffer
            .iter()
            .enumerate()
            .fold_while((self.actual_current, -1), |acc, (index, current)| {
                if let Some(current) = current {
                    FoldWhile::Continue((*current, index as i32))
                } else {
                    FoldWhile::Done(acc)
                }
            })
            .into_inner()
    }

    fn actual_position(&self, lerp: f32) -> Vec3 {
        self.actual_last.lerp(self.actual_current, lerp)
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
        let mut buffer = [None; BUFFER_SIZE];
        buffer[0] = Some(network_object.position);

        commands.entity(entity).insert(NetworkPhysicsBuffer {
            buffer,
            current_network_index: network_object.update_index,
            actual_current: network_object.position,
            actual_last: network_object.position,
        });
    }
}

pub fn update_network_physics_buffer(
    smoothings: Query<(&mut NetworkPhysicsBuffer, Ref<NetworkPhysicsObject>)>,
) {
    for (mut network_buffer, network_object) in smoothings {
        if network_object.is_changed() {
            network_buffer.set_position(
                network_object.update_index,
                network_object.position,
            );
        }

        network_buffer.try_pop();
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
