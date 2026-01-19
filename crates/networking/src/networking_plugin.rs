use bevy::{ecs::system::NonSendMarker, prelude::*};
use bevy_hookup_core::{
    hookup_component_plugin::HookupComponentPlugin,
    hookup_sendable_plugin::HookupSendablePlugin,
    reshare_component_plugin::ReshareComponentPlugin,
    reshare_entity_plugin::ReshareEntityPlugin,
    share_component::ShareComponent,
    sync_entity::SyncEntityOwner,
    utils::{
        buffer_object::BufferObject, buffer_plugin::BufferPlugin,
        component_buffer::ComponentBuffer,
    },
};
use bevy_hookup_messenger_steamworks::{
    steamworks_client::SteamworksClient, steamworks_server::SteamworksServer,
    steamworks_server_plugin::SteamworksServerPlugin,
    steamworks_session_handler_plugin::SteamworksSessionHandlerPlugin,
};
use bevy_hookup_messenger_websocket::{
    websocket_client::WebsocketClient,
    websocket_client_plugin::WebsocketClientPlugin,
    websocket_client_state::WebsocketClientState,
    websocket_server::WebsocketServer,
    websocket_server_plugin::WebsocketServerPlugin,
};
use bevy_steamworks::{Client, LobbyType, SteamworksEvent};
use physics::physics_position::PhysicsPosition;
use player::player_component::PlayerRotation;
use steamworks::CallbackResult;
use world_generation::{
    generation_options::GenerationOptions, start_world_gen::StartWorldGen,
};

use crate::{
    create_world::CreateWorld, networking_state::NetworkingState,
    sendables::Sendables, start_self_session::StartSelfSession,
    start_steam_server::StartSteamServer,
    start_websocket_client::StartWebsocketClient,
    start_websocket_server::StartWebsocketServer,
};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<BufferObject<PhysicsPosition>>()
        .register_type::<ComponentBuffer<PhysicsPosition, 4>>()
        .add_plugins((
            WebsocketClientPlugin::<Sendables>::default(),
            WebsocketServerPlugin::<Sendables>::default(),
            SteamworksServerPlugin::<Sendables>::default(),
            SteamworksSessionHandlerPlugin::<Sendables>::default(),
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, GenerationOptions>::default(),
            HookupComponentPlugin::<Sendables, PlayerRotation>::default(),
            BufferPlugin::<Sendables, PhysicsPosition, 4>::default(),
            ReshareEntityPlugin::<Sendables>::default(),
            ReshareComponentPlugin::<BufferObject<PhysicsPosition>>::default(),
            ReshareComponentPlugin::<PlayerRotation>::default(),
        ))
        .init_state::<NetworkingState>()
        .add_systems(Update, (check_steamworks_events, client_on_connect))
        .add_observer(start_self_session)
        .add_observer(create_world)
        .add_observer(start_websocket_server)
        .add_observer(start_steam_server)
        .add_observer(start_websocket_client);
    }
}

fn check_steamworks_events(
    mut events: MessageReader<SteamworksEvent>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<NetworkingState>>,
    state: Res<State<NetworkingState>>,
    client: Res<Client>,
) {
    for event in events.read() {
        match event {
            SteamworksEvent::CallbackResult(callback_result) => {
                match callback_result {
                    CallbackResult::GameLobbyJoinRequested(a) => {
                        if *state.get() != NetworkingState::Off {
                            continue;
                        }

                        if let Err(err) = SteamworksClient::<Sendables>::create(
                            &client,
                            a.friend_steam_id,
                            &mut commands,
                        ) {
                            error!(
                                "Failed to create steamworks client: {}",
                                err
                            );
                        } else {
                            commands.trigger(StartWorldGen);
                            next_state.set(NetworkingState::Client);
                        }

                        client.matchmaking().join_lobby(
                            a.lobby_steam_id,
                            |result| match result {
                                Ok(_) => {
                                    info!("Joined lobby!");
                                }
                                Err(_) => {
                                    error!("Failed to join lobby!");
                                }
                            },
                        );
                        info!("Lobby join requested!");
                    }
                    _ => {}
                }
            }
        }
    }
}

fn start_self_session(_: On<StartSelfSession>, mut commands: Commands) {
    commands.trigger(StartWorldGen);
}

fn start_websocket_client(
    event: On<StartWebsocketClient>,
    mut commands: Commands,
    _: NonSendMarker,
) {
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
    mut state: ResMut<NextState<NetworkingState>>,
    mut commands: Commands,
) {
    let (entity, ws_state) = ws_state.into_inner();

    match ws_state {
        WebsocketClientState::Connected => {
            state.set(NetworkingState::Client);
            commands.trigger(StartWorldGen);
        }
        WebsocketClientState::Failed => {
            warn!("Failed to connect!");
            commands.entity(entity).despawn();
        }
        _ => {}
    }
}

fn start_websocket_server(
    _: On<StartWebsocketServer>,
    mut commands: Commands,
    mut state: ResMut<NextState<NetworkingState>>,
    _: NonSendMarker,
) {
    state.set(NetworkingState::Host);
    commands.spawn(WebsocketServer::<Sendables>::new_with_port(1324));
    commands.trigger(StartWorldGen);
}

fn start_steam_server(
    _: On<StartSteamServer>,
    client: Res<Client>,
    mut commands: Commands,
    mut state: ResMut<NextState<NetworkingState>>,
    _: NonSendMarker,
) {
    state.set(NetworkingState::Host);

    client.matchmaking().create_lobby(
        LobbyType::FriendsOnly,
        12,
        move |result| match result {
            Ok(lobby_id) => {
                info!("Successfully created lobby with id: {:?}", lobby_id);
            }
            Err(err) => {
                error!("Failed creating lobby: {}", err);
            }
        },
    );

    commands.spawn(
        SteamworksServer::<Sendables>::new(&client)
            .expect("Couldn't create steamworks server"),
    );

    commands.trigger(StartWorldGen);
}

fn create_world(event: On<CreateWorld>, mut commands: Commands) {
    commands.spawn((
        SyncEntityOwner::new(),
        GenerationOptions::from_seed(event.seed),
        ShareComponent::<GenerationOptions>::default(),
    ));
}
