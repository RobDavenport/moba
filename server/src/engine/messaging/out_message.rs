use crate::engine::components::all::{PlayerId, ReplicationId};
use std::cmp::Ordering;

//Messages that are broadcasted from the Server to Game Clients only
//Change this to a struct? that implements toProtobuf ?
#[derive(Clone, Debug)]
pub enum OutMessage {
    Snapshot {
        baseline: Option<u32>,
        frame: u32,
        entities: Vec<EntitySnapshot>,
    },
    UpdateTick {
        frame: u32,
        entitySnapshot: EntitySnapshot,
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
    pub replication_id: ReplicationId,
    pub x: NetworkedFloat,
    pub y: NetworkedFloat,
    pub rotation: NetworkedFloat,
}

impl Ord for EntitySnapshot {
    fn cmp(&self, other: &Self) -> Ordering {
        self.replication_id.cmp(&other.replication_id)
    }
}

impl PartialOrd for EntitySnapshot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EntitySnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.replication_id == other.replication_id && self.x == other.x && self.y == other.y
    }
}

impl Eq for EntitySnapshot {}

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

impl PartialEq for NetworkedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.round() as i32 == other.0.round() as i32
    }
}

impl Eq for NetworkedFloat {}

use crate::engine::network::protobuf::ServerMessage::*;
use protobuf::Message as Message_imported_for_functions;
use protobuf::RepeatedField;

impl OutMessage {
    pub fn to_proto_bytes(self) -> Vec<u8> {
        match self {
            Self::UpdateTick {
                frame,
                entitySnapshot,
            } => update_tick(frame, entitySnapshot),
            Self::VerifyUuid(uuid) => verify_uuid(uuid),
            Self::VerifiedUuid => verified_uuid(),
            Self::EntityDestroyed {
                frame,
                replication_id,
            } => entity_destroyed(frame, replication_id),
            Self::Snapshot {
                frame,
                entities,
                baseline,
            } => snapshot(frame, entities, baseline),
        }
    }
}

fn update_tick(
    frame: u32,
    entitySnapshot: EntitySnapshot,
) -> Vec<u8> {
    let mut output = ServerMessage::new();

    let mut inner = ServerMessage_UpdateTick::new();
    inner.set_frame(frame);

    let mut entityData = ServerMessage_EntityData::new();
    entityData.set_replicationId(entitySnapshot.replication_id.0);
    entityData.set_x(entitySnapshot.x.into());
    entityData.set_y(entitySnapshot.y.into());
    entityData.set_rotation(entitySnapshot.rotation.into());

    inner.set_entityData(entityData);

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

fn snapshot(frame: u32, entities: Vec<EntitySnapshot>, baseline: Option<u32>) -> Vec<u8> {
    let mut output = ServerMessage::new();

    let mut inner = ServerMessage_Snapshot::new();
    inner.set_frame(frame);
    if let Some(base) = baseline {
        inner.set_baseline(base);
    }

    inner.set_entityData(RepeatedField::from_vec(
        entities
            .into_iter()
            .map(|entity| {
                let mut single_data = ServerMessage_EntityData::new();
                single_data.set_x(entity.x.into());
                single_data.set_y(entity.y.into());
                single_data.set_replicationId(entity.replication_id.0);
                single_data.set_rotation(entity.rotation.into());
                single_data
            })
            .collect(),
    ));

    output.set_snapshot(inner);
    output.write_to_bytes().unwrap()
}
