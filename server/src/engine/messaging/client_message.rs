extern crate ws;

use super::messages::GameMessage;
use crate::engine::network::ws::client_data::ClientData;

// For messages from a Client Socket to the Client Manager
#[derive(Debug)]
pub enum ClientMessage {
    Connected(ClientData),
    Disconnected(u32),
    GameMessage(GameMessage),
    ChatMessage {
        id: u32,
        public: bool,
        message: String,
    },
}
