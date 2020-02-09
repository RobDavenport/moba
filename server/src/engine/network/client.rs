use std::sync::mpsc;

extern crate ws;
use ws::*;

use super::client_message::ClientMessage;

pub struct Client {
    pub manager_out: mpsc::Sender<ClientMessage>,
    pub out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Client: connected.");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.manager_out.send(ClientMessage::ChatMessage {
            public: false,
            message: msg.to_string(),
        });

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, _reason: &str) {
        println!("Client: Closed reason: {:?}", code);
        self.manager_out
            .send(ClientMessage::Disconnected(self.out.token()));
    }
}
