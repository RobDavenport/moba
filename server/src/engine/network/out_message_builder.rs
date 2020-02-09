//use serde::*;

use super::out_message::OutMessage;

pub fn build_message(msg: OutMessage) -> String {
  match msg {
    OutMessage::UpdateTick {x, y} => {
      "INVALID STRING".to_string()
      //serde_json::to_string(&msg).unwrap()
    }
  }
}