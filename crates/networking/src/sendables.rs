use bevy_hookup_macros::Sendable;

use world_generation::generation_options::GenerationOptions;

#[derive(Clone, Sendable)]
pub enum Sendables {
    #[sendable]
    GenerationOptions(GenerationOptions),
}
