use bevy::prelude::*;
use bevy_hookup_core::utils::{
    buffer_systems::BufferSystems, buffered::Buffered,
};

use crate::{
    physics_position::PhysicsPosition,
    physics_previous_position::PhysicsPreviousPosition,
    physics_systems::PhysicsSystems, update_physics::update_physics,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (update_physics.in_set(PhysicsSystems),))
            .add_systems(
                Update,
                (
                    update_transform_position.in_set(PhysicsSystems),
                    update_buffered_physics,
                ),
            )
            .add_systems(
                FixedUpdate,
                update_buffered_previous
                    .after(BufferSystems::<PhysicsPosition>::default()),
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

fn update_buffered_physics(
    buffered: Query<
        (
            &Buffered<PhysicsPosition>,
            &PhysicsPreviousPosition,
            &mut Transform,
        ),
        Without<PhysicsPosition>,
    >,
    fixed_time: Res<Time<Fixed>>,
) {
    for (buffered, previous, mut transform) in buffered {
        transform.translation =
            previous.lerp(***buffered, fixed_time.overstep_fraction());
    }
}

fn update_buffered_previous(
    buffered: Query<
        (
            Entity,
            &Buffered<PhysicsPosition>,
            Option<&mut PhysicsPreviousPosition>,
        ),
        Without<PhysicsPosition>,
    >,
    mut commands: Commands,
) {
    for (entity, buffered, previous) in buffered {
        if let Some(mut previous) = previous {
            **previous = ***buffered;
        } else {
            commands
                .entity(entity)
                .insert(PhysicsPreviousPosition(***buffered));
        }
    }
}
