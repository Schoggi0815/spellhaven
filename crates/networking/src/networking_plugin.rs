use bevy::prelude::*;
use bevy_hookup_core::{
    hookup_component_plugin::HookupComponentPlugin,
    hookup_sendable_plugin::HookupSendablePlugin,
};
use world_generation::generation_options::GenerationOptions;

use crate::sendables::Sendables;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, GenerationOptions>::default(),
        ));
    }
}
