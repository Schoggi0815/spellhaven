use bevy::{ecs::system::NonSendMarker, prelude::*};
use bevy_hookup_core::{
    hookup_component_plugin::HookupComponentPlugin,
    hookup_sendable_plugin::HookupSendablePlugin,
    reshare_component_plugin::ReshareComponentPlugin,
    reshare_entity_plugin::ReshareEntityPlugin,
    share_component::ShareComponent, sync_entity::SyncEntityOwner,
};
use bevy_hookup_messenger_steamworks::{
    steamworks_client::SteamworksClient, steamworks_server::SteamworksServer,
    steamworks_server_plugin::SteamworksServerPlugin,
    steamworks_session_handler_plugin::SteamworksSessionHandlerPlugin,
};
use bevy_steamworks::{Client, LobbyType, SteamworksEvent};
use physics::network_physics_object::NetworkPhysicsObject;
use player::player_component::PlayerRotation;
use steamworks::CallbackResult;
use world_generation::{
    generation_options::GenerationOptions, start_world_gen::StartWorldGen,
};

use crate::{
    create_world::CreateWorld, networking_state::NetworkingState,
    sendables::Sendables, start_self_session::StartSelfSession,
    start_websocket_server::StartWebsocketServer,
};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SteamworksServerPlugin::<Sendables>::default(),
            SteamworksSessionHandlerPlugin::<Sendables>::default(),
            HookupSendablePlugin::<Sendables>::default(),
            HookupComponentPlugin::<Sendables, GenerationOptions>::default(),
            HookupComponentPlugin::<Sendables, PlayerRotation>::default(),
            HookupComponentPlugin::<Sendables, NetworkPhysicsObject>::default(),
            ReshareEntityPlugin::<Sendables>::default(),
            ReshareComponentPlugin::<NetworkPhysicsObject>::default(),
            ReshareComponentPlugin::<PlayerRotation>::default(),
        ))
        .init_state::<NetworkingState>()
        .add_systems(Update, check_steamworks_events)
        .add_observer(start_self_session)
        .add_observer(create_world)
        .add_observer(start_websocket_server);
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

fn start_websocket_server(
    _: On<StartWebsocketServer>,
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
