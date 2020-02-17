use tokio::sync::mpsc::Sender;

use ws;

extern crate serde_json;

use super::client::Client;

use crate::engine::messaging::messages::ClientMessage;
use crate::engine::network::client_data::ClientData;

pub struct ClientFactory {
    client_sender: Sender<ClientMessage>,
    next_client_id: u32,
}

impl ClientFactory {
    pub fn new(client_sender: Sender<ClientMessage>) -> Self {
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
                ws_client_out: out.clone(),
                socket_addr: None,
            }));

        new_client
    }
}
