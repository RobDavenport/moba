use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum OutMessage {
  UpdateTick {x: f32, y: f32},
}