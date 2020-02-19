use serde::{Deserialize, Serialize};

//Messages that are broadcasted from the Server to Game Clients only
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "t", content = "d")]
//#[serde(tag = "type")]
pub enum OutMessage {
    UpdateTick { f: u32, x: f32, y: f32, n: u32 },
    VerifyUuid(String),
    VerifiedUuid,
}

pub enum OutTarget {
    All,
    Single(u32),
    Many(Vec<u32>),
}
