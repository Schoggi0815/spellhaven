use bevy::{ecs::system::NonSendMarker, prelude::*};
use bevy_hookup_core::{
    hook_session::SessionMessenger,
    hookup_component_plugin::HookupComponentPlugin,
    hookup_sendable_plugin::HookupSendablePlugin, owner_component::Owner,
    reshare_component_plugin::ReshareComponentPlugin,
    reshare_entity_plugin::ReshareEntityPlugin, sync_entity::SyncEntityOwner,
};
use bevy_hookup_messenger_self::self_session::SelfSession;
use bevy_hookup_messenger_websocket::{
    websocket_client::WebsocketClient,
    websocket_client_plugin::WebsocketClientPlugin,
    websocket_client_state::WebsocketClientState,
    websocket_server::WebsocketServer,
    websocket_server_plugin::WebsocketServerPlugin,
};
use physics::network_physics_object::NetworkPhysicsObject;
use player::player_component::PlayerRotation;
use world_generation::{
    generation_options::GenerationOptions, start_world_gen::StartWorldGen,
};

use crate::{
    create_world::CreateWorld, sendables::Sendables,
    start_self_session::StartSelfSession,
    start_websocket_client::StartWebsocketClient,
    start_websocket_server::StartWebsocketServer,
};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WebsocketClientPlugin::<Sendables>::default(),
            WebsocketServerPlugin::<Sendables>::default(),
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, GenerationOptions>::default(),
            HookupComponentPlugin::<Sendables, PlayerRotation>::default(),
            HookupComponentPlugin::<Sendables, NetworkPhysicsObject>::default(),
            ReshareEntityPlugin::<Sendables>::default(),
            // ReshareComponentPlugin::<Sendables, PlayerPosition>::default(),
        ))
        .add_systems(Update, client_on_connect)
        .add_observer(start_self_session)
        .add_observer(create_world)
        .add_observer(start_websocket_client)
        .add_observer(start_websocket_server);
    }
}

fn start_self_session(_: On<StartSelfSession>, mut commands: Commands) {
    commands.spawn(SelfSession::<Sendables>::new().to_session());
    commands.trigger(StartWorldGen);
}

fn start_websocket_server(
    _: On<StartWebsocketServer>,
    mut commands: Commands,
    _: NonSendMarker,
) {
    commands.spawn(WebsocketServer::<Sendables>::new_with_port(1324));
    commands.spawn(SelfSession::<Sendables>::new().to_session());
    commands.trigger(StartWorldGen);
}

fn start_websocket_client(
    event: On<StartWebsocketClient>,
    mut commands: Commands,
    _: NonSendMarker,
) {
    commands.spawn(SelfSession::<Sendables>::new().to_session());
    commands.spawn(WebsocketClient::<Sendables>::new_with_host_and_port(
        event.address.clone(),
        1324,
    ));
}

fn client_on_connect(
    ws_state: Single<
        (Entity, &WebsocketClientState),
        Changed<WebsocketClientState>,
    >,
    mut commands: Commands,
) {
    let (entity, ws_state) = ws_state.into_inner();

    match ws_state {
        WebsocketClientState::Connected => {
            commands.trigger(StartWorldGen);
        }
        WebsocketClientState::Failed => {
            warn!("Failed to connect!");
            commands.entity(entity).despawn();
        }
        _ => {}
    }
}

fn create_world(event: On<CreateWorld>, mut commands: Commands) {
    commands.spawn((
        SyncEntityOwner::new(),
        Owner::new(GenerationOptions::from_seed(event.seed)),
    ));
}
