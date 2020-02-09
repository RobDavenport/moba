use std::sync::mpsc;

extern crate ws;
use ws::*;

use super::client;
use super::client_message::ClientMessage;
use crate::engine::game_message::GameMessage;

pub struct ClientManager {
    sender: mpsc::Sender<ClientMessage>,
    game_channel: mpsc::Sender<GameMessage>,
}

impl ClientManager {
    pub fn new(game_channel: mpsc::Sender<GameMessage>) -> Self {
        let (sender, receiver) = mpsc::channel::<ClientMessage>();
        let my_channel = game_channel.clone();
        let mut client_outs: Vec<ws::Sender> = Vec::new();

        std::thread::spawn(move || {
            loop {
                match receiver.try_recv() {
                    Ok(msg) => match msg {
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
                    },
                    Err(_) => (),
                }
            }
        });

        Self {
            game_channel: my_channel,
            sender,
        }
    }
}

impl Factory for ClientManager {
    type Handler = client::Client;

    fn connection_made(&mut self, out: Sender) -> client::Client {
        self.sender
            .send(ClientMessage::Connected(out.clone()))
            .unwrap();
            
        client::Client {
            manager_out: self.sender.clone(),
            out,
        }
    }
}
