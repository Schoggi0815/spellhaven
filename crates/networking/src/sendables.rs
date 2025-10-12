use bevy::transform::components::Transform;
use bevy_hookup_macros::Sendable;

use player::player_component::Player;
use serde::{Deserialize, Serialize};
use world_generation::generation_options::GenerationOptions;

#[derive(Clone, Sendable, Serialize, Deserialize)]
pub enum Sendables {
    #[sendable]
    GenerationOptions(GenerationOptions),
    #[sendable]
    Player(Player),
    #[sendable]
    Transform(Transform),
}
