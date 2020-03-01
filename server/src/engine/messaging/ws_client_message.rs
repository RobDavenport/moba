extern crate ws;

use crate::engine::components::player_controlled::PlayerId;
use crate::engine::network::client_data::ClientData;

// Messages which come from client (Ws)
#[derive(Debug)]
pub enum WSClientMessage {
    Connected(PlayerId, ClientData),
    Disconnected(PlayerId),
    Packet(PlayerId, Vec<u8>),
}
