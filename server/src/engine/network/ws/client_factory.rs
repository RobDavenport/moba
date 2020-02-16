// use std::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender, channel};

use ws;

extern crate serde_json;

use super::client::Client;
use super::client_data::ClientData;
use super::client_manager_looper::ClientManagerLooper;

use crate::engine::messaging::messages::{ClientMessage, GameMessage, OutMessage};

pub struct ClientFactory {
    client_sender: Sender<ClientMessage>,
    next_client_id: u32,
}

impl ClientFactory {
    pub fn new(
        game_channel: Sender<GameMessage>,
        out_receiver: Receiver<OutMessage>,
        channel_size: usize
    ) -> Self {
        let (client_sender, client_receiver) = channel::<ClientMessage>(channel_size);

        std::thread::spawn(move || {
            ClientManagerLooper::new(client_receiver, out_receiver, game_channel).start_loop();
        });

        Self {
            client_sender,
            next_client_id: 0,
        }
    }
}

impl ws::Factory for ClientFactory {
    type Handler = Client;

    fn connection_made(&mut self, out: ws::Sender) -> Client {
        let new_client = Client {
            manager_out: self.client_sender.clone(),
            id: self.next_client_id,
        };

        self.next_client_id += 1;
        self.client_sender
            .try_send(ClientMessage::Connected(ClientData {
                id: new_client.id,
                client_out: out.clone(),
            }));

        new_client
    }
}
