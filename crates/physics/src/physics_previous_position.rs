use bevy::prelude::*;

#[derive(Component, Default, Deref, DerefMut)]
pub struct PhysicsPreviousPosition(pub Vec3);
