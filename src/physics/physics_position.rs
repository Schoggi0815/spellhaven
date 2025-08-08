use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PhysicsPosition {
    pub position: Vec3,
    pub velocity: Vec3,
}
