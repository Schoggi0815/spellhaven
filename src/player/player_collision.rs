use avian3d::{math::Scalar, parry::simba::scalar::SupersetOf, prelude::*};
use bevy::prelude::*;

fn kinematic_collision(
    collisions: Collisions,
    mut bodies: Query<(&RigidBody, &mut Position, &Rotation)>,
) {
    // // Iterate through collisions and move the kinematic body to resolve penetration
    // for contacts in collisions.iter() {
    //     // If the collision didn't happen during this substep, skip the collision
    //     if !contacts.is_in_subset() {
    //         continue;
    //     }
    //     if let Ok([(rb1, mut position1, rotation1), (rb2, mut position2, _)]) =
    //         bodies.get_many_mut([contacts.collider1, contacts.collider2])
    //     {
    //         let Some(contact) = contacts.find_deepest_contact() else {
    //             continue;
    //         };

    //         if contact.penetration <= Scalar::EPSILON {
    //             continue;
    //         }
    //         if rb1.is_kinematic() && !rb2.is_kinematic() {
    //             position1.0 -= contact
    //                 .global_point1(position, rotation)
    //                 .global_normal1(rotation1)
    //                 * contact.penetration;
    //         } else if rb2.is_kinematic() && !rb1.is_kinematic() {
    //             position2.0 +=
    //                 contact.global_normal1(rotation1) * contact.penetration;
    //         }

    //         for manifold in contacts.manifolds.iter() {
    //             for contact in manifold.contacts.iter() {}
    //         }
    //     }
    // }
}
