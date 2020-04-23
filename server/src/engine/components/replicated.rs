use crate::engine::game::Game;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Replicated {
    pub id: ReplicationId,
}

impl Replicated {
    pub fn new_for_game(game: &mut Game) -> Self {
        Self {
            id: game.get_new_replication_id(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReplicationId(pub u32);
