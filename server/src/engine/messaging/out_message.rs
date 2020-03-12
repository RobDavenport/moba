use crate::engine::components::all::{PlayerId, ReplicationId};

//Messages that are broadcasted from the Server to Game Clients only
//Change this to a struct? that implements toProtobuf ?
#[derive(Clone, Debug)]
pub enum OutMessage {
    Snapshot {
        frame: u32,
        entities: Vec<EntitySnapshot>,
    },
    UpdateTick {
        frame: u32,
        x: NetworkedFloat,
        y: NetworkedFloat,
        replication_id: ReplicationId,
    },
    EntityDestroyed {
        frame: u32,
        replication_id: ReplicationId,
    },
    VerifyUuid(String),
    VerifiedUuid,
}

#[derive(Clone, Debug)]
pub struct EntitySnapshot {
    pub x: NetworkedFloat,
    pub y: NetworkedFloat,
    pub replication_id: ReplicationId,
}

pub enum OutTarget {
    All,
    Single(PlayerId),
    Many(Vec<PlayerId>),
}

#[derive(Clone, Debug)]
pub struct NetworkedFloat(f32);

impl From<f32> for NetworkedFloat {
    fn from(item: f32) -> Self {
        NetworkedFloat(item)
    }
}

impl Into<i32> for NetworkedFloat {
    fn into(self) -> i32 {
        self.0.round() as i32
    }
}

use crate::engine::network::protobuf::ServerMessage::*;
use protobuf::Message as Message_imported_for_functions;
use protobuf::RepeatedField;

impl OutMessage {
    pub fn to_proto_bytes(self) -> Vec<u8> {
        match self {
            Self::UpdateTick {
                frame,
                x,
                y,
                replication_id,
            } => update_tick(frame, x, y, replication_id),
            Self::VerifyUuid(uuid) => verify_uuid(uuid),
            Self::VerifiedUuid => verified_uuid(),
            Self::EntityDestroyed {
                frame,
                replication_id,
            } => entity_destroyed(frame, replication_id),
            Self::Snapshot { frame, entities } => snapshot(frame, entities),
        }
    }
}

fn update_tick(
    frame: u32,
    x: NetworkedFloat,
    y: NetworkedFloat,
    replication_id: ReplicationId,
) -> Vec<u8> {
    let mut output = ServerMessage::new();

    let mut inner = ServerMessage_UpdateTick::new();
    inner.set_frame(frame);
    inner.set_replicationId(replication_id.0);
    inner.set_x(x.into());
    inner.set_y(y.into());

    output.set_updateTick(inner);
    output.write_to_bytes().unwrap()
}

fn verify_uuid(uuid: String) -> Vec<u8> {
    let mut output = ServerMessage::new();

    let mut inner = ServerMessage_VerifyUuid::new();
    inner.set_uuid(uuid);

    output.set_verifyUuid(inner);
    output.write_to_bytes().unwrap()
}

fn verified_uuid() -> Vec<u8> {
    let mut output = ServerMessage::new();
    output.set_verifiedUuid(ServerMessage_VerifiedUuid::new());
    output.write_to_bytes().unwrap()
}

fn entity_destroyed(frame: u32, replication_id: ReplicationId) -> Vec<u8> {
    let mut output = ServerMessage::new();

    let mut inner = ServerMessage_EntityDestroyed::new();
    inner.set_frame(frame);
    inner.set_replicationId(replication_id.0);

    output.set_entityDestroyed(inner);
    output.write_to_bytes().unwrap()
}

fn snapshot(frame: u32, entities: Vec<EntitySnapshot>) -> Vec<u8> {
    let mut output = ServerMessage::new();

    let mut inner = ServerMessage_Snapshot::new();
    inner.set_frame(frame);

    inner.set_entityData(RepeatedField::from_vec(
        entities
            .into_iter()
            .map(|entity| {
                let mut single_data = ServerMessage_EntityData::new();
                single_data.set_x(entity.x.into());
                single_data.set_y(entity.y.into());
                single_data.set_replicationId(entity.replication_id.0);
                single_data
            })
            .collect(),
    ));

    output.set_snapshot(inner);
    output.write_to_bytes().unwrap()
}
