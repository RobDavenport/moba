use crate::engine::components::{player_controlled::PlayerId, replicated::ReplicationId};

pub enum GameEvent {
    EntityDestroyed(ReplicationId),
    ClientDisconnected(PlayerId),
}
