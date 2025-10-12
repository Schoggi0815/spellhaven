use bevy::prelude::*;
use physics::physics_set::PhysicsSet;

use crate::{
    player_camera_movement::move_camera,
    player_component::{spawn_player, spawn_player_body},
    player_movement::{move_body, movement},
    player_state::PlayerState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .add_systems(Update, (move_camera, move_body))
            .add_systems(Update, movement.after(PhysicsSet))
            .add_observer(spawn_player)
            .add_observer(spawn_player_body);
    }
}
