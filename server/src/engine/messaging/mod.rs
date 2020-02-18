mod game_message;
mod out_message;
mod ws_client_message;

pub mod messages {
    pub use super::game_message::GameMessage;
    pub use super::out_message::{OutMessage, OutTarget};
    pub use super::ws_client_message::WSClientMessage;
}
