//Messages that are broadcasted from the Server to Game Clients only
#[derive(Clone, Debug)]
pub enum OutMessage {
    UpdateTick {
        frame: u32,
        x: f32,
        y: f32,
        entity: u32,
    },
    VerifyUuid(String),
    VerifiedUuid,
}

pub enum OutTarget {
    All,
    Single(u32),
    Many(Vec<u32>),
}
