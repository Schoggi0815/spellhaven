use bevy::prelude::*;

use physics::{
    collider::Collider,
    collider_trait::ColliderTrait,
    physics_object::{DynamicPhysicsObject, StaticPhysicsObject},
    physics_position::PhysicsPosition,
};

#[derive(Resource, Default)]
pub struct PhysicsDebugResource;

pub fn render_physics_debug(
    mut gizmos: Gizmos,
    colliders_dynamic: Query<
        (&Collider, &PhysicsPosition),
        With<DynamicPhysicsObject>,
    >,
    colliders_static: Query<(&Collider, &Transform), With<StaticPhysicsObject>>,
) {
    for (collider, position) in colliders_dynamic {
        for aabb in collider.get_aabbs() {
            gizmos.cuboid(
                Transform::from_translation(**position + aabb.offset)
                    .with_scale(aabb.size),
                Color::hsl(100., 1., 0.5),
            );
        }
    }

    for (collider, transform) in colliders_static {
        let min_distance = colliders_dynamic
            .iter()
            .map(|d| d.1.distance_squared(transform.translation).ceil() as i32)
            .min();

        let Some(min_distance) = min_distance else {
            return;
        };

        if min_distance > 32i32.pow(2) {
            continue;
        }

        for aabb in collider.get_aabbs() {
            gizmos.cuboid(
                Transform::from_translation(
                    transform.translation + aabb.offset,
                )
                .with_scale(aabb.size),
                Color::hsl(200., 1., 0.5),
            );
        }
    }
}
