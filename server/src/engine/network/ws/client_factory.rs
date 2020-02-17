use tokio::sync::mpsc::Sender;

use uuid::Uuid;
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

        let uuid = Uuid::new_v4().to_string();

        self.next_client_id += 1;
        self.client_sender
            .try_send(ClientMessage::Connected(ClientData {
                id: new_client.id,
                ws_client_out: out.clone(),
                socket_addr: None,
                socket_uuid: uuid.clone(),
            }));

        println!("sending uuid: {}", &uuid);
        out.send(format!("{{\"uuid\":\"{}\"}}", &uuid));

        new_client
    }
}
