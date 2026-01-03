use std::f32::consts::PI;

use bevy::prelude::*;
use physics::physics_systems::PhysicsSystems;

use crate::{
    player_camera_movement::move_camera,
    player_component::{
        PlayerBody, PlayerRotation, spawn_player, spawn_player_body,
    },
    player_inputs::{PlayerInputs, update_player_inputs},
    player_movement::movement,
    player_state::PlayerState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .init_resource::<PlayerInputs>()
            .add_systems(Update, (move_camera, spawn_player_body))
            .add_systems(PreUpdate, update_player_inputs)
            .add_systems(FixedUpdate, movement.before(PhysicsSystems))
            .add_systems(Update, (rotate_body_smoothed, zoom))
            .add_observer(spawn_player);
    }
}

fn rotate_body_smoothed(
    player_bodies: Query<(&mut Transform, &PlayerRotation), With<PlayerBody>>,
) {
    for (mut transform, player_rotation) in player_bodies {
        transform.rotation = transform.rotation.lerp(**player_rotation, 0.25);
    }
}

fn zoom(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut Projection>,
) {
    let Projection::Perspective(perspective) = player.as_mut() else {
        return;
    };

    if keyboard_input.just_pressed(KeyCode::KeyC) {
        perspective.fov = PI / 16.;
    }
    if keyboard_input.just_released(KeyCode::KeyC) {
        perspective.fov = PI / 4.;
    }
}
