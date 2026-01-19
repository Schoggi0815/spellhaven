use bevy::prelude::*;

use crate::{
    physics_position::PhysicsPosition,
    physics_previous_position::PhysicsPreviousPosition,
    physics_velocity::PhysicsVelocity,
};

#[derive(Component)]
pub struct StaticPhysicsObject;

#[derive(Component, Default)]
#[require(PhysicsPosition, PhysicsVelocity, PhysicsPreviousPosition)]
pub struct DynamicPhysicsObject {
    pub step_height: f32,
    pub touching_sides: IVec3,
}
