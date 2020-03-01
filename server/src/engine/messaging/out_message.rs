use crate::engine::components::all::{PlayerId, ReplicationId};

//Messages that are broadcasted from the Server to Game Clients only
#[derive(Clone, Debug)]
pub enum OutMessage {
    UpdateTick {
        frame: u32,
        x: f32,
        y: f32,
        replication_id: ReplicationId,
    },
    VerifyUuid(String),
    VerifiedUuid,
}

pub enum OutTarget {
    All,
    Single(PlayerId),
    Many(Vec<PlayerId>),
}
