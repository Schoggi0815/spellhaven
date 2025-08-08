use bevy::prelude::*;

use crate::physics::{physics_set::PhysicsSet, update_physics::update_physics};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_physics.in_set(PhysicsSet));
    }
}
