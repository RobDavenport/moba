extern crate ws;
use ws::*;
use ws::util::Token;

use super::client;

pub struct WsServer {
  senders: Vec<Sender>
}

impl WsServer {
  pub fn new() -> Self {
      Self {
          senders: Vec::new()
      }
  }

  pub fn add_user(&mut self, sender: Sender) {
      self.senders.push(sender)
  }

  pub fn broadcast_all(&self, msg: Message) {
      for sender in self.senders.iter() {
          sender.send(msg.clone());
      }
  }

  pub fn remove_user(&mut self, token: Token) {
      self.senders.retain(|out| token != out.token());
  }
}

impl Factory for WsServer {
  type Handler = client::Client;

  fn connection_made(&mut self, ws: Sender) -> client::Client {
      self.add_user(ws.clone());
      client::Client {
        server: self,
          out: ws
      }
  }
}

