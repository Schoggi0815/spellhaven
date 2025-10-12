use bevy::prelude::*;
use bevy_hookup_core::{
    hook_session::SessionMessenger,
    hookup_component_plugin::HookupComponentPlugin,
    hookup_sendable_plugin::HookupSendablePlugin, owner_component::Owner,
    sync_entity::SyncEntityOwner,
};
use bevy_hookup_messenger_self::self_session::SelfSession;
use world_generation::{
    generation_options::GenerationOptions, start_world_gen::StartWorldGen,
};

use crate::{
    create_world::CreateWorld, sendables::Sendables,
    start_self_session::StartSelfSession,
};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, GenerationOptions>::default(),
        ))
        .add_observer(start_self_session)
        .add_observer(create_world);
    }
}

fn start_self_session(_: On<StartSelfSession>, mut commands: Commands) {
    commands.spawn(SelfSession::<Sendables>::new().to_session());
    commands.trigger(StartWorldGen);
}

fn create_world(event: On<CreateWorld>, mut commands: Commands) {
    commands.spawn((
        SyncEntityOwner::new(),
        Owner::new(GenerationOptions::from_seed(event.seed)),
    ));
}
