use bevy::prelude::*;
use bevy_hookup_core::{
    owner_component::Owner, send_component_set::SendComponentSet,
    shared::Shared,
};
use physics::physics_set::PhysicsSet;

use crate::{
    player_camera_movement::move_camera,
    player_component::{
        Player, PlayerBody, PlayerPosition, PlayerSmoothing, spawn_player,
        spawn_player_body,
    },
    player_movement::movement,
    player_state::PlayerState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_systems(Update, (move_camera, spawn_player_body))
            .add_systems(Update, movement.after(PhysicsSet))
            .add_systems(
                Update,
                update_player_pos
                    .before(SendComponentSet::<PlayerPosition>::default()),
            )
            .add_systems(
                Update,
                (
                    update_player_smoothing,
                    rotate_body_smoothed,
                    move_player_smoothed.after(update_player_smoothing),
                ),
            )
            .add_observer(spawn_player);
    }
}

fn update_player_pos(
    mut player: Single<(&mut Owner<PlayerPosition>, &Transform)>,
    time: Res<Time>,
) {
    let velocity = ((player.1.translation - player.0.position)
        / time.delta_secs())
    .round();

    if player.0.position == player.1.translation
        && player.0.rotation == player.1.rotation
        && player.0.velocity == velocity
    {
        return;
    }

    player.0.position = player.1.translation;
    player.0.rotation = player.1.rotation;
    player.0.velocity = velocity;
}

fn update_player_smoothing(
    other_players: Query<
        (&Shared<PlayerPosition>, &mut PlayerSmoothing, &Transform),
        Changed<Shared<PlayerPosition>>,
    >,
) {
    for (player_position, mut player_smoothing, transfrom) in other_players {
        info!("New Velocity: {}", player_position.velocity);
        player_smoothing.lerp_time = 0.;
        player_smoothing.start_pos = transfrom.translation;
        player_smoothing.end_pos =
            player_position.position + player_position.velocity * 0.2;
    }
}

fn rotate_body_smoothed(
    player_bodies: Query<
        (&mut Transform, &Shared<PlayerPosition>),
        With<PlayerBody>,
    >,
) {
    for (mut transform, player_position) in player_bodies {
        transform.rotation =
            transform.rotation.lerp(player_position.rotation, 0.25);
    }
}

fn move_player_smoothed(
    player_smoothings: Query<(&mut PlayerSmoothing, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut player_smoothing, mut transform) in player_smoothings {
        player_smoothing.lerp_time += time.delta_secs() * 5.;
        let new_pos = player_smoothing
            .start_pos
            .lerp(player_smoothing.end_pos, player_smoothing.lerp_time.min(1.));
        transform.translation = new_pos;
    }
}
