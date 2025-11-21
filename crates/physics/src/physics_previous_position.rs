use bevy::prelude::*;

#[derive(Component, Default, Deref, DerefMut)]
pub struct PhysicsPreviousPosition(Vec3);
