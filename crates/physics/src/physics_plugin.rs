use bevy::prelude::*;
use bevy_hookup_core::receive_component_set::ReceiveComponentSet;
use bevy_hookup_core::send_component_systems::SendComponentSystems;

use crate::{
    network_physics_object::{NetworkPhysicsObject, update_network_physics},
    network_physics_smoothing::{
        add_smoothing, move_network_physics_smoothed,
        update_network_physics_smoothing,
    },
    physics_position::PhysicsPosition,
    physics_previous_position::PhysicsPreviousPosition,
    physics_systems::PhysicsSystems,
    update_physics::update_physics,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                update_physics.in_set(PhysicsSystems),
                update_network_physics.after(PhysicsSystems).before(
                    SendComponentSystems::<NetworkPhysicsObject>::default(),
                ),
                update_network_physics_smoothing
                    .after(
                        ReceiveComponentSet::<NetworkPhysicsObject>::default(),
                    ),
                add_smoothing
                    .after(
                        ReceiveComponentSet::<NetworkPhysicsObject>::default(),
                    )
                    .before(update_network_physics_smoothing),
            ),
        )
        .add_systems(
            Update,
            (
                update_transform_position.in_set(PhysicsSystems),
                move_network_physics_smoothed,
            ),
        );
    }
}

pub fn update_transform_position(
    positions: Query<(
        &PhysicsPosition,
        &PhysicsPreviousPosition,
        &mut Transform,
    )>,
    fixed_time: Res<Time<Fixed>>,
) {
    for (physics_position, previous_position, mut transform) in positions {
        transform.translation = previous_position
            .lerp(**physics_position, fixed_time.overstep_fraction());
    }
}
