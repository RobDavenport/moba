extern crate ws;
use ws::*;
use ws::util::Token;

use super::client;

use crate::engine::engine::Engine;

pub struct WsServer {
  senders: Vec<Sender>,
  engine: Engine
}

impl WsServer {
  pub fn new(engine: Engine) -> Self {
      Self {
          senders: Vec::new(),
          engine
      }
  }

  pub fn add_user(&mut self, sender: Sender) {
      self.engine.player_connected();
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

