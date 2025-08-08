use bevy::math::Vec3;

use crate::physics::aabb_collider::AabbCollider;

pub trait ColliderTrait {
    fn is_colliding_with(
        &self,
        self_position: Vec3,
        other_collider: &impl ColliderTrait,
        other_position: Vec3,
    ) -> bool;

    fn restrict_movement(
        &self,
        end_position: Vec3,
        other_collider: &impl ColliderTrait,
        other_position: Vec3,
    ) -> Vec3;

    fn get_aabbs<'a>(&'a self) -> Vec<&'a AabbCollider>;
}
