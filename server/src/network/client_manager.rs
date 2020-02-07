use std::sync::mpsc;

extern crate ws;
use ws::*;
use ws::util::Token;

use super::client;
use super::client_message::ClientMessage;
use crate::engine::game_message::GameMessage;

pub struct ClientManager {
  client_outs: Vec<ws::Sender>,
  game_channel: mpsc::Sender<GameMessage>,
  receiver: mpsc::Receiver<ClientMessage>,
  sender: mpsc::Sender<ClientMessage>,
}

impl ClientManager {
  pub fn new(game_channel: mpsc::Sender<GameMessage>) -> Self {
    let (sender, receiver) = mpsc::channel::<ClientMessage>();

    Self {
      client_outs: Vec::new(),
      game_channel,
      sender,
      receiver
    }
  }

  pub fn add_user(&mut self, sender: Sender) {
    //TODO RECEIVE THIS SOMEWHERE
      self.game_channel.send(GameMessage::ClientConnected);
      self.client_outs.push(sender)
  }

  pub fn broadcast_all(&self, msg: Message) {
      for out in self.client_outs.iter() {
          out.send(msg.clone());
      }
  }

  pub fn remove_user(&mut self, token: Token) {
      self.client_outs.retain(|out| token != out.token());
  }
}

impl Factory for ClientManager {
  type Handler = client::Client;

  fn connection_made(&mut self, out: Sender) -> client::Client {
      self.add_user(out.clone());
      client::Client {
        manager_out: self.sender.clone(),
        out,
      }
  }
}

