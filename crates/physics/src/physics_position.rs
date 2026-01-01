use bevy::prelude::*;
use bevy_hookup_core::utils::interpolate::Interpolate;
use serde::{Deserialize, Serialize};

#[derive(
    Component,
    Reflect,
    Serialize,
    Deserialize,
    Default,
    Deref,
    DerefMut,
    Clone,
    Copy,
    Debug,
    PartialEq,
)]
pub struct PhysicsPosition(pub Vec3);

impl Interpolate for PhysicsPosition {
    fn interpolate(&self, other: &Self, percentage: f32) -> Self {
        PhysicsPosition(self.lerp(**other, percentage))
    }
}
