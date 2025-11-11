use bevy::prelude::*;
use itertools::Itertools;

use crate::{
    collider::Collider,
    collider_trait::ColliderTrait,
    physics_object::{DynamicPhysicsObject, StaticPhysicsObject},
    physics_position::PhysicsPosition,
    physics_previous_position::PhysicsPreviousPosition,
    physics_velocity::PhysicsVelocity,
};

pub fn update_physics(
    static_objects: Query<(&StaticPhysicsObject, &Collider, &Transform)>,
    dynamic_objects: Query<(
        &Collider,
        &mut PhysicsPosition,
        &mut PhysicsVelocity,
        &mut PhysicsPreviousPosition,
        &mut DynamicPhysicsObject,
    )>,
    time: Res<Time>,
) {
    let all_colliders = static_objects
        .iter()
        .map(|s| (s.1, s.2.translation))
        .collect_vec();

    for (
        dynamic_collider,
        mut dynamic_position,
        mut dynamic_velocity,
        mut dynamic_previous_position,
        mut dynamic_object,
    ) in dynamic_objects
    {
        dynamic_object.touching_sides = IVec3::ZERO;

        let velocity = **dynamic_velocity * time.delta_secs();

        let mut new_pos = **dynamic_position + velocity;

        let colliding_statics = static_objects.iter().filter(|so| {
            dynamic_collider.is_colliding_with(new_pos, so.1, so.2.translation)
        });

        for (_, static_collider, static_transform) in colliding_statics
            .sorted_by(|a, b| {
                a.2.translation
                    .distance_squared(**dynamic_position)
                    .total_cmp(
                        &b.2.translation.distance_squared(**dynamic_position),
                    )
            })
        {
            new_pos = dynamic_collider.restrict_movement(
                new_pos,
                static_collider,
                static_transform.translation,
                &all_colliders,
                dynamic_object.step_height,
                &mut dynamic_object.touching_sides,
            );
        }

        if dynamic_object.touching_sides.x != 0 {
            dynamic_velocity.x = 0.;
        }

        if dynamic_object.touching_sides.y != 0 {
            dynamic_velocity.y = 0.;
        }

        if dynamic_object.touching_sides.z != 0 {
            dynamic_velocity.z = 0.;
        }

        **dynamic_previous_position = **dynamic_position;
        **dynamic_position = new_pos;
    }
}
