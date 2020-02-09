use std::sync::mpsc;
use std::sync::mpsc::*;

extern crate ws;
use ws::*;

extern crate serde_json;
use serde_json::*;

use super::client;
use super::client_message::ClientMessage;
use crate::engine::game_message::GameMessage;

use super::out_message::OutMessage;

const SLEEP_NANO_SECONDS: u64 = 1;

pub struct ClientManager {
    client_sender: mpsc::Sender<ClientMessage>,
    game_channel: mpsc::Sender<GameMessage>,
}

impl ClientManager {
    pub fn new(game_channel: mpsc::Sender<GameMessage>, out_receiver: mpsc::Receiver<OutMessage>) -> Self {
        let (client_sender, client_receiver) = mpsc::channel::<ClientMessage>();
        let my_channel = game_channel.clone();
        let mut client_outs: Vec<ws::Sender> = Vec::new();

        std::thread::spawn(move || {
            let sleep_duration = std::time::Duration::from_nanos(SLEEP_NANO_SECONDS);
            loop {
                //Match client messages...
                match client_receiver.try_recv() {
                    Ok(msg) => {
                        match msg {
                            ClientMessage::MoveCommand { x, y } => {
                                game_channel
                                    .send(GameMessage::MoveCommand { x, y })
                                    .unwrap();
                            }
                            ClientMessage::Connected(in_client) => {
                                client_outs.push(in_client.clone());
                                //TODO do something with in_client??
                                game_channel.send(GameMessage::ClientConnected).unwrap();
                            }
                            ClientMessage::Disconnected(token) => {
                                client_outs.retain(|client| token != client.token());
                            }
                            ClientMessage::ChatMessage { public: _, message } => {
                                for out in client_outs.iter() {
                                    out.send(message.clone()).unwrap();
                                }
                            }
                            _ => {
                                println!("ClientManager: Unhandled message!");
                            }
                        }
                    },
                    Err(e) => {
                        match e {
                            TryRecvError::Empty => (),
                            TryRecvError::Disconnected => println!("Client Disconnected!"),
                        }
                    },
                };

                //Match mesages from game
                match out_receiver.try_recv() {
                    Ok(msg) => {
                        match msg {
                            OutMessage::UpdateTick { .. } => {
                                let output = serde_json::to_string(&msg).unwrap();
                                for out in client_outs.iter() {
                                    out.send(output.clone()).unwrap();
                                }
                            }
                        }
                    },
                                                    // for out in client_outs.iter() {
                                //     out.send(output).unwrap();
                                // }
                    Err(e) => {
                        match e {
                            TryRecvError::Empty => (),
                            TryRecvError::Disconnected => println!("Game + Client Manager breakdown!")
                        }
                    }
                }

                std::thread::sleep(sleep_duration);
            }
        });

        Self {
            game_channel: my_channel,
            client_sender,
        }
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
