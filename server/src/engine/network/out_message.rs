use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutMessage {
  UpdateTick {x: f32, y: f32},
}