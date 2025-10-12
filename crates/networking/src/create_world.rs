use bevy::prelude::*;

#[derive(Event)]
pub struct CreateWorld {
    pub seed: u64,
}
