use bevy_hookup_macros::Sendable;

use physics::network_physics_object::NetworkPhysicsObject;
use player::player_component::PlayerRotation;
use serde::{Deserialize, Serialize};
use world_generation::generation_options::GenerationOptions;

#[derive(Clone, Sendable, Serialize, Deserialize, Debug)]
pub enum Sendables {
    #[sendable]
    GenerationOptions(GenerationOptions),
    #[sendable]
    PlayerRotation(PlayerRotation),
    #[sendable]
    NetworkPhysicsObject(NetworkPhysicsObject),
}
