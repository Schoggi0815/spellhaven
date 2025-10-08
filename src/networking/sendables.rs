use bevy_hookup_macros::Sendable;

use crate::world_generation::generation_options::GenerationOptions;

#[derive(Clone, Sendable)]
pub enum Sendables {
    #[sendable]
    GenerationOptions(GenerationOptions),
}
