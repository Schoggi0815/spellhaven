use bevy::prelude::*;
use bevy_hookup_core::{
    owner_component::Owner, send_component_systems::SendComponentSystems,
    shared::Shared,
};
use physics::physics_systems::PhysicsSystems;

use crate::{
    player_camera_movement::move_camera,
    player_component::{
        PlayerBody, PlayerRotation, spawn_player, spawn_player_body,
    },
    player_movement::movement,
    player_state::PlayerState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_systems(Update, (move_camera, spawn_player_body))
            .add_systems(PreUpdate, movement)
            .add_systems(
                FixedUpdate,
                update_rotation
                    .after(PhysicsSystems)
                    .before(SendComponentSystems::<PlayerRotation>::default()),
            )
            .add_systems(Update, rotate_body_smoothed)
            .add_observer(spawn_player);
    }
}

fn update_rotation(player: Single<(&mut Owner<PlayerRotation>, &Transform)>) {
    let (mut player_rotation, transform) = player.into_inner();

    if ***player_rotation == transform.rotation {
        return;
    }

    ***player_rotation = transform.rotation;
}

fn rotate_body_smoothed(
    player_bodies: Query<
        (&mut Transform, &Shared<PlayerRotation>),
        With<PlayerBody>,
    >,
) {
    for (mut transform, player_rotation) in player_bodies {
        transform.rotation = transform.rotation.lerp(***player_rotation, 0.25);
    }
}
