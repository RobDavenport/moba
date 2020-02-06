extern crate ws;
use ws::*;

use super::ws_server::WsServer;

pub struct Client { 
  pub server: *mut WsServer,
  pub out: Sender
}

impl Handler for Client {
  fn on_open(&mut self, _: Handshake) -> Result<()> {
      println!("User connected successfully...");
      Ok(())
  }

  fn on_message(&mut self, msg: Message) -> Result<()> {
      println!("Got message: {}", msg);
      unsafe {
          self.server.as_ref().unwrap().broadcast_all(msg);
      }
      Ok(())
  }

  fn on_close(&mut self, code: CloseCode, _reason: &str) {
      println!("User left because: {:?}", code);
      unsafe {
          self.server.as_mut().unwrap().remove_user(self.out.token());
      }
  }
}