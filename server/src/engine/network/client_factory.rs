use std::sync::mpsc;

extern crate ws;
use ws::*;

extern crate serde_json;

use super::client::Client;
use super::client_data::ClientData;
use super::client_message::ClientMessage;
use crate::engine::game_message::GameMessage;

use super::client_manager_looper::ClientManagerLooper;
use super::out_message::OutMessage;

pub struct ClientFactory {
    client_sender: mpsc::Sender<ClientMessage>,
    next_client_id: u32,
}

impl ClientFactory {
    pub fn new(
        game_channel: mpsc::Sender<GameMessage>,
        out_receiver: mpsc::Receiver<OutMessage>,
    ) -> Self {
        let (client_sender, client_receiver) = mpsc::channel::<ClientMessage>();

        std::thread::spawn(move || {
            ClientManagerLooper::new(client_receiver, out_receiver, game_channel).start_loop();
        });

        Self {
            client_sender,
            next_client_id: 0,
        }
    }
}

impl Factory for ClientFactory {
    type Handler = Client;

    fn connection_made(&mut self, out: ws::Sender) -> Client {
        let new_client = Client {
            manager_out: self.client_sender.clone(),
            id: self.next_client_id,
        };

        self.next_client_id += 1;
        self.client_sender
            .send(ClientMessage::Connected(ClientData {
                id: new_client.id,
                client_out: out.clone(),
            }))
            .unwrap();

        new_client
    }
}
