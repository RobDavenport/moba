extern crate ws;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;

use crate::engine::messaging::messages::{ClientMessage, GameMessage, OutMessage};
use crate::engine::network::client_data::ClientData;

use futures::{future::FutureExt, select, stream::StreamExt};

pub struct ClientManagerLooper {
    //todo delete this struct, and merge into network manager
    client_listener: Receiver<ClientMessage>,
    game_out_listener: Receiver<OutMessage>,
    game_channel: Sender<GameMessage>,
    client_data: Vec<ClientData>,
}

impl ClientManagerLooper {
    pub fn spawn_client_looper(
        client_listener: Receiver<ClientMessage>,
        game_out_listener: Receiver<OutMessage>,
        mut game_channel: Sender<GameMessage>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut manager = ClientManagerLooper {
                client_listener,
                game_out_listener,
                game_channel,
                client_data: Vec::new(),
            };

            loop {
                select! {
                    client_msg = manager.client_listener.recv().fuse() => {
                        manager.handle_client_message(client_msg);
                    }
                    game_out = manager.game_out_listener.recv().fuse() => { // TODO remove this
                        manager.handle_game_out_message(game_out);
                    }
                }
            }
        })
    }

    fn handle_client_message(&mut self, msg: Option<ClientMessage>) {
        match msg {
            None => print!("invalid client message received"),
            Some(client_msg) => match client_msg {
                ClientMessage::Connected(in_client_data) => {
                    self.client_data.push(in_client_data);
                    self.game_channel
                        .try_send(GameMessage::ClientConnected)
                        .unwrap();
                }
                ClientMessage::Disconnected(disconnect_id) => {
                    self.client_data.retain(|client| disconnect_id != client.id);
                }
                ClientMessage::GameMessage(game_message) => {
                    self.game_channel.try_send(game_message).unwrap();
                }
                ClientMessage::ChatMessage {
                    id,
                    public: _,
                    message,
                } => self.broadcast_message(message),
            },
        }
    }

    fn handle_game_out_message(&self, msg: Option<OutMessage>) {
        //Todo remove this
        match msg {
            None => println!("invalid out message received"),
            Some(out_msg) => match out_msg {
                OutMessage::UpdateTick { .. } => {
                    let output = serde_json::to_string(&out_msg).unwrap();
                    self.broadcast_message(output);
                }
            },
        }
    }

    fn broadcast_message(&self, msg: String) {
        for data in self.client_data.iter() {
            data.ws_client_out.send(msg.clone()).unwrap();
        }
    }
}
