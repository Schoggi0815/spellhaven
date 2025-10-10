use bevy::prelude::*;
use physics::physics_set::PhysicsSet;
use ui::main_menu_state::MainMenuState;

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
            .add_systems(
                OnEnter(MainMenuState::Hidden),
                (spawn_player, spawn_player_body),
            )
            .add_systems(Update, movement.after(PhysicsSet));
    }
}
