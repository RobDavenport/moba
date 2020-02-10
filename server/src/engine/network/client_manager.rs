use std::sync::mpsc;

extern crate ws;
use ws::*;

extern crate serde_json;

use super::client;
use super::client_message::ClientMessage;
use crate::engine::game_message::GameMessage;

use super::client_manager_looper::ClientManagerLooper;
use super::out_message::OutMessage;

pub struct ClientManager {
    client_sender: mpsc::Sender<ClientMessage>,
}

impl ClientManager {
    pub fn new(
        game_channel: mpsc::Sender<GameMessage>,
        out_receiver: mpsc::Receiver<OutMessage>,
    ) -> Self {
        let (client_sender, client_receiver) = mpsc::channel::<ClientMessage>();
        std::thread::spawn(move || {
            ClientManagerLooper::new(client_receiver, out_receiver, game_channel).start_loop();
        });

        Self { client_sender }
    }
}

impl Factory for ClientManager {
    type Handler = client::Client;

    fn connection_made(&mut self, out: ws::Sender) -> client::Client {
        self.client_sender
            .send(ClientMessage::Connected(out.clone()))
            .unwrap();

        client::Client {
            manager_out: self.client_sender.clone(),
            out,
        }
    }
}
