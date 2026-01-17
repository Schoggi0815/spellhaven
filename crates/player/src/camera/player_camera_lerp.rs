use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PlayerCameraLerp {
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for PlayerCameraLerp {
    fn default() -> Self {
        Self {
            distance: 10.,
            yaw: 0.,
            pitch: 0.,
        }
    }
}
