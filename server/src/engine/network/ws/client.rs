use tokio::sync::mpsc::Sender;
use ws::*;

use crate::engine::components::player_controlled::PlayerId;
use crate::engine::messaging::messages::WSClientMessage;

pub struct Client {
    pub manager_out: Sender<WSClientMessage>,
    pub id: PlayerId,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("Client {{{}}} connected.", self.id);
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.manager_out
            .try_send(WSClientMessage::Packet(self.id, msg.into_data()));

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, _reason: &str) {
        println!("Client {{{}}} Closed reason: {:?}", self.id, code);
        self.manager_out
            .try_send(WSClientMessage::Disconnected(self.id));
    }
}
