extern crate ws;

use crate::engine::network::client_data::ClientData;
use super::messages::GameMessage;

// For messages from a Client Socket to the Client Manager
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
