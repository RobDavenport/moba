use crate::engine::components::{player_controlled::PlayerId, replicated::ReplicationId};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameEvent {
    EntityDestroyed(ReplicationId),
    ClientDisconnected(PlayerId),
}
