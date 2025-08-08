use bevy::prelude::*;
use itertools::Itertools;

use crate::physics::{
    aabb_collider::AabbCollider, collider_trait::ColliderTrait,
};

pub struct CompoundCollider {
    pub colliders: Vec<AabbCollider>,
}

impl ColliderTrait for CompoundCollider {
    fn is_colliding_with(
        &self,
        self_position: Vec3,
        other_collider: &impl ColliderTrait,
        other_position: Vec3,
    ) -> bool {
        for self_collider in self.get_aabbs() {
            if self_collider.is_colliding_with(
                self_position,
                other_collider,
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

        for self_collider in
            other_collider.get_aabbs().iter().sorted_by(|aabb1, aabb2| {
                aabb1
                    .distance_squared(other_position, end_position)
                    .total_cmp(
                        &aabb2.distance_squared(other_position, end_position),
                    )
            })
        {
            result = self_collider.restrict_movement(
                end_position,
                other_collider,
                other_position,
            );
        }

        result
    }

    fn get_aabbs<'a>(&'a self) -> Vec<&'a AabbCollider> {
        self.colliders.iter().collect_vec()
    }
}
