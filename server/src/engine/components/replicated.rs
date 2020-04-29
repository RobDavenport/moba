use crate::engine::game::Game;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Replicated {
    pub id: ReplicationId,
    pub entity_type: ReplicatedEntityType,
}

impl Replicated {
    pub fn new_for_game(game: &mut Game, entity_type: ReplicatedEntityType) -> Self {
        Self {
            id: game.get_new_replication_id(),
            entity_type,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReplicationId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReplicatedEntityType {
    Character,
    Minion,
    Tower,
    Core
}