use std::sync::mpsc;
use std::sync::mpsc::*;

extern crate ws;

use super::client_message::ClientMessage;
use super::out_message::OutMessage;
use crate::engine::game_message::GameMessage;
use crate::engine::message_listener::MessageListener;

const SLEEP_NANO_SECONDS: u64 = 1;

pub struct ClientManagerLooper {
    client_listener: MessageListener<ClientMessage>,
    game_out_listener: MessageListener<OutMessage>,
    game_channel: mpsc::Sender<GameMessage>,
    client_outs: Vec<ws::Sender>,
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
            client_outs: Vec::new(),
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
            // ClientMessage::MoveCommand { x, y } => {
            //     println!("GOT MOVE COMMAND!");
            //     self.game_channel
            //         .send(GameMessage::MoveCommand { x: *x, y: *y })
            //         .unwrap();
            // }
            ClientMessage::Connected(in_client) => {
                self.client_outs.push(in_client.clone());
                //TODO do something with in_client??
                self.game_channel
                    .send(GameMessage::ClientConnected)
                    .unwrap();
            }
            ClientMessage::Disconnected(token) => {
                self.client_outs.retain(|client| token != client.token());
            }
            ClientMessage::GameMessage(game_message) => {
              //TODO Pull the value out of game_message somewhere?
              //Find out difference between copy/clone here
              self.game_channel
                //.send(game_message.clone())
                .send(GameMessage::MoveCommand {x: 5., y: 5.,})
                .unwrap();
            }
            ClientMessage::ChatMessage { public: _, message } => {
                //TODO make "broadcast function"
                for out in self.client_outs.iter() {
                    out.send(message.clone()).unwrap();
                }
            }
        }
    }

    fn handle_game_out_message(&self, msg: OutMessage) {
        match msg {
            OutMessage::UpdateTick { .. } => {
                let output = serde_json::to_string(&msg).unwrap();
                for out in self.client_outs.iter() {
                    out.send(output.clone()).unwrap();
                }
            }
        }
    }
}
