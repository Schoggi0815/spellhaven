use bevy::prelude::*;

use crate::utils::velocity::update_velocity;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_velocity);
    }
}
