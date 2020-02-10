extern crate ws;

use crate::engine::game_message::GameMessage;

// For messages from a Client Socket to the Client Manager
pub enum ClientMessage {
    Connected(ws::Sender),
    Disconnected(ws::util::Token),
    GameMessage(GameMessage),
    ChatMessage { public: bool, message: String },
}
