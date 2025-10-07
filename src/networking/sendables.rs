use bevy_hookup_macros::Sendable;

use crate::world_generation::generation_options::GenerationOptions;

#[derive(Debug, Clone, Sendable)]
pub enum Sendables {
    GenerationOptions(GenerationOptions),
}
