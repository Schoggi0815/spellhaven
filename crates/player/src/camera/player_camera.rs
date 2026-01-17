use bevy::prelude::*;

use crate::camera::player_camera_lerp::PlayerCameraLerp;

#[derive(Component, Debug)]
#[require(PlayerCameraLerp)]
pub struct PlayerCamera {
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub target_pos: Vec3,
    pub target_offset: Vec3,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            distance: 10.,
            yaw: 0.,
            pitch: 0.,
            target_pos: Vec3::ZERO,
            target_offset: Vec3::ZERO,
        }
    }
}
