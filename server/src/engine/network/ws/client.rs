

use tokio::sync::mpsc::Sender;
use ws::*;

use crate::engine::messaging::messages::ClientMessage;

pub struct Client {
    pub manager_out: Sender<ClientMessage>,
    pub id: u32,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Client: connected.");
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        //TODO Translate messages into usable ClientMessage format...
        self.manager_out
            .try_send(ClientMessage::ChatMessage {
                id: self.id,
                public: false,
                message: msg.to_string(),
            });

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, _reason: &str) {
        println!("Client: Closed reason: {:?}", code);
        self.manager_out
            .try_send(ClientMessage::Disconnected(self.id));
    }
}
