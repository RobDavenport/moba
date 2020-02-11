use std::sync::mpsc;
use std::sync::mpsc::*;

extern crate ws;

use super::client_data::ClientData;
use super::client_message::ClientMessage;
use super::out_message::OutMessage;
use crate::engine::game_message::GameMessage;
use crate::engine::message_listener::MessageListener;

const SLEEP_NANO_SECONDS: u64 = 1;

pub struct ClientManagerLooper {
    client_listener: MessageListener<ClientMessage>,
    game_out_listener: MessageListener<OutMessage>,
    game_channel: mpsc::Sender<GameMessage>,
    client_data: Vec<ClientData>,
}

impl ClientManagerLooper {
    pub fn new(
        client_receiver: Receiver<ClientMessage>,
        out_receiver: Receiver<OutMessage>,
        game_channel: mpsc::Sender<GameMessage>,
    ) -> Self {
        Self {
            client_listener: MessageListener::new(client_receiver),
            game_out_listener: MessageListener::new(out_receiver),
            game_channel,
            client_data: Vec::new(),
        }
    }

    pub fn start_loop(&mut self) {
        let sleep_duration = std::time::Duration::from_nanos(SLEEP_NANO_SECONDS);

        loop {
            //Match client messages...
            if let Some(mut client_messages) = self.client_listener.check_messages() {
                for client_message in client_messages.drain(..) {
                    self.handle_client_message(client_message)
                }
            }

            //Match mesages from game
            if let Some(mut game_out_messages) = self.game_out_listener.check_messages() {
                for game_out_message in game_out_messages.drain(..) {
                    self.handle_game_out_message(game_out_message)
                }
            }

            std::thread::sleep(sleep_duration);
        }
    }

    fn handle_client_message(&mut self, msg: ClientMessage) {
        match msg {
            ClientMessage::Connected(in_client_data) => {
                self.client_data.push(in_client_data);
                self.game_channel
                    .send(GameMessage::ClientConnected)
                    .unwrap();
            }
            ClientMessage::Disconnected(disconnect_id) => {
                self.client_data.retain(|client| disconnect_id != client.id);
            }
            ClientMessage::GameMessage(game_message) => {
                self.game_channel.send(game_message).unwrap();
            }
            ClientMessage::ChatMessage {
                id,
                public: _,
                message,
            } => self.broadcast_message(message),
        }
    }

    fn handle_game_out_message(&self, msg: OutMessage) {
        match msg {
            OutMessage::UpdateTick { .. } => {
                let output = serde_json::to_string(&msg).unwrap();
                self.broadcast_message(output);
            }
        }
    }

    fn broadcast_message(&self, msg: String) {
        for data in self.client_data.iter() {
            data.client_out.send(msg.clone()).unwrap();
        }
    }
}
