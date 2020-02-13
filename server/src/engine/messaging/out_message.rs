use serde::{Deserialize, Serialize};

//Messages that are broadcasted from the Server to Game Clients only
#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(tag = "t", content = "d")]
pub enum OutMessage {
    UpdateTick { f: u32, x: f32, y: f32 },
}
