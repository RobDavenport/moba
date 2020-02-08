extern crate ws;
use ws::*;

pub enum ClientMessage {
    Connected(ws::Sender),
    Disconnected(ws::util::Token),

    MoveCommand { x: f32, y: f32 },
    ChatMessage { public: bool, message: String },
}
