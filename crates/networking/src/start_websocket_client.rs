use bevy::prelude::*;

#[derive(Event)]
pub struct StartWebsocketClient {
    pub address: String,
}
