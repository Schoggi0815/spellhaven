use bevy::prelude::*;

use crate::physics::{
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
                Transform::from_translation(position.position + aabb.offset)
                    .with_scale(aabb.size),
                Color::hsl(100., 1., 0.5),
            );
        }
    }

    for (collider, transform) in colliders_static {
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
