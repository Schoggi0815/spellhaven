use bevy::prelude::*;

use crate::physics::physics_position::PhysicsPosition;

#[derive(Component)]
pub struct StaticPhysicsObject;

#[derive(Component)]
#[require(PhysicsPosition)]
pub struct DynamicPhysicsObject;
