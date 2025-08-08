use bevy::prelude::*;
use itertools::Itertools;

use crate::physics::collider_trait::ColliderTrait;

#[derive(Clone)]
pub struct AabbCollider {
    pub size: Vec3,
    pub offset: Vec3,
}

impl ColliderTrait for AabbCollider {
    fn is_colliding_with(
        &self,
        self_position: Vec3,
        other_collider: &impl ColliderTrait,
        other_position: Vec3,
    ) -> bool {
        for other_collider in other_collider.get_aabbs() {
            if self.is_colliding_with_aabb(
                self_position,
                &other_collider,
                other_position,
            ) {
                return true;
            }
        }

        false
    }

    fn restrict_movement(
        &self,
        end_position: Vec3,
        other_collider: &impl ColliderTrait,
        other_position: Vec3,
    ) -> Vec3 {
        let mut result = end_position;

        for other_collider in
            other_collider.get_aabbs().iter().sorted_by(|aabb1, aabb2| {
                aabb1
                    .distance_squared(other_position, end_position)
                    .total_cmp(
                        &aabb2.distance_squared(other_position, end_position),
                    )
            })
        {
            result = self.restrict_movement_aabb(
                result,
                &other_collider,
                other_position,
            );
        }

        result
    }

    fn get_aabbs<'a>(&'a self) -> Vec<&'a AabbCollider> {
        vec![self]
    }
}

impl AabbCollider {
    pub fn is_colliding_with_aabb(
        &self,
        self_position: Vec3,
        other_collider: &AabbCollider,
        other_position: Vec3,
    ) -> bool {
        let distance = ((self_position + self.offset)
            - (other_position + other_collider.offset))
            .abs();
        let collision_distance = (self.size + other_collider.size) / 2.;

        distance.x < collision_distance.x
            && distance.y < collision_distance.y
            && distance.z < collision_distance.z
    }

    pub fn restrict_movement_aabb(
        &self,
        end_position: Vec3,
        other_collider: &AabbCollider,
        other_position: Vec3,
    ) -> Vec3 {
        let distance = (other_position + other_collider.offset)
            - (end_position + self.offset);
        let collider_offset = (self.size / 2.) + (other_collider.size / 2.);

        let intersection = distance.abs() - collider_offset;

        if !(intersection.max_element() < 0.) {
            return end_position;
        }

        if intersection.abs().x == intersection.abs().min_element() {
            return end_position
                - (Vec3::X * intersection.x.copysign(distance.x));
        }

        if intersection.abs().y == intersection.abs().min_element() {
            return end_position
                - (Vec3::Y * intersection.y.copysign(distance.y));
        }

        if intersection.abs().z == intersection.abs().min_element() {
            return end_position
                - (Vec3::Z * intersection.z.copysign(distance.z));
        }

        end_position
    }

    pub fn distance_squared(
        &self,
        self_position: Vec3,
        other_position: Vec3,
    ) -> f32 {
        ((self_position + self.offset) - other_position).length_squared()
    }
}
