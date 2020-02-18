extern crate ws;

use super::messages::GameMessage;
use crate::engine::network::client_data::ClientData;

// Messages which come from client (Ws)
#[derive(Debug)]
pub enum WSClientMessage {
    Connected(ClientData),
    Disconnected(u32),
    Packet(u32, Vec<u8>),
}
