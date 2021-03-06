use crate::engine::components::all::{PlayerId, ReplicatedEntityType, ReplicationId};
use std::cmp::Ordering;

//Messages that are broadcasted from the Server to Game Clients only
//Change this to a struct? that implements toProtobuf ?
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum OutMessage {
    Snapshot {
        baseline: Option<u32>,
        frame: u32,
        entities: Vec<EntitySnapshot>,
    },
    UpdateTick {
        frame: u32,
        entity_snapshot: EntitySnapshot,
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
    pub x: Option<NetworkedFloat>,
    pub y: Option<NetworkedFloat>,
    pub rotation: Option<NetworkedFloat>,
    pub health: Option<NetworkedFloat>,
    pub energy: Option<NetworkedFloat>,
    pub entity_type: Option<ReplicatedEntityType>,
    // TODO Add EntityState from Protobuf
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
        self.replication_id == other.replication_id
            && self.x == other.x
            && self.y == other.y
            && self.rotation == other.rotation
            && self.health == other.health
            && self.energy == other.energy
            && self.entity_type == other.entity_type
    }
}

impl Eq for EntitySnapshot {}

#[allow(dead_code)]
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
    pub fn into_proto_bytes(self) -> Vec<u8> {
        match self {
            Self::UpdateTick {
                frame,
                entity_snapshot,
            } => update_tick(frame, entity_snapshot),
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

fn update_tick(frame: u32, entity_snapshot: EntitySnapshot) -> Vec<u8> {
    let mut output = ServerMessage::new();

    let mut inner = ServerMessage_UpdateTick::new();
    inner.set_frame(frame);

    let entity_data = entity_snapshot_into_proto_msg(entity_snapshot);

    inner.set_entityData(entity_data);

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
            .map(entity_snapshot_into_proto_msg)
            .collect(),
    ));

    output.set_snapshot(inner);
    output.write_to_bytes().unwrap()
}

fn entity_snapshot_into_proto_msg(entity: EntitySnapshot) -> ServerMessage_EntityData {
    let mut single_data = ServerMessage_EntityData::new();
    single_data.set_replicationId(entity.replication_id.0);
    if let Some(x) = entity.x {
        single_data.set_x(x.into())
    };
    if let Some(y) = entity.y {
        single_data.set_y(y.into())
    };
    if let Some(rot) = entity.rotation {
        single_data.set_rotation(rot.into())
    };
    if let Some(health) = entity.health {
        single_data.set_health(health.into())
    };
    if let Some(energy) = entity.energy {
        single_data.set_energy(energy.into())
    };

    // TODO add state
    //if let Some(state) = entity.state {
    //
    //}

    if let Some(entity_type) = entity.entity_type {
        let mut type_data = ServerMessage_EntityTypeData::new();

        //TODO Finish these as more data comes
        match entity_type {
            ReplicatedEntityType::Character => {
                type_data.set_characterData(ServerMessage_EntityTypeData_CharacterData::new())
            }
            ReplicatedEntityType::Minion => {
                type_data.set_minionData(ServerMessage_EntityTypeData_MinionData::new())
            }
            ReplicatedEntityType::Tower => {
                type_data.set_towerData(ServerMessage_EntityTypeData_TowerData::new())
            }
            ReplicatedEntityType::Core => {
                type_data.set_coreData(ServerMessage_EntityTypeData_CoreData::new())
            }
        };

        single_data.set_entityTypeData(type_data);
    };
    single_data
}
