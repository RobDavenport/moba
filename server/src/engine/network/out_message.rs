use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum OutMessage {
    UpdateTick { frame: u32, x: f32, y: f32 },
}
