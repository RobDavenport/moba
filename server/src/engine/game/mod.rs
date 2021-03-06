use std::collections::{BinaryHeap, HashMap, VecDeque};

use legion::{prelude::*, world::World};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::engine::components::all::*;
use crate::engine::events::{game_event::GameEvent, timed_event::TimedEvent};
use crate::engine::messaging::messages::{GameMessage, OutMessage, OutTarget};
use crate::engine::network::delta_encoder::SnapshotHistory;

use crate::engine::input_command::*;

pub mod archetypes;
pub mod game_loop;
pub mod init;
pub mod input;
use input::InputQueue;

pub mod prelude {
    pub use super::archetypes::*;
    pub use super::game_loop::*;
    pub use super::init::*;
    pub use super::input::*;
}

pub struct Game {
    tick_time: f32,
    world: World,
    game_time: f32,
    game_frame: u32,
    out_reliable: Sender<(OutTarget, OutMessage)>,
    out_unreliable: Sender<(OutTarget, OutMessage)>,
    game_in: Receiver<GameMessage>,
    player_entities: HashMap<PlayerId, Entity>,
    player_snapshot_histories: HashMap<PlayerId, SnapshotHistory>,
    player_inputs: HashMap<PlayerId, InputQueue>,
    replicated_entities: HashMap<ReplicationId, Entity>,
    replication_counter: u32,
    game_events: VecDeque<GameEvent>,
    timed_events: BinaryHeap<TimedEvent>,
    executor: Executor,
}

impl Game {
    pub fn new(
        tick_time: f32,
        out_reliable: Sender<(OutTarget, OutMessage)>,
        out_unreliable: Sender<(OutTarget, OutMessage)>,
        game_in: Receiver<GameMessage>,
    ) -> Self {
        Self {
            tick_time,
            world: Universe::new().create_world(),
            game_time: 0.,
            game_frame: 0,
            out_reliable,
            out_unreliable,
            game_in,
            player_entities: HashMap::new(),
            replication_counter: 0,
            game_events: VecDeque::new(),
            player_snapshot_histories: HashMap::new(),
            executor: Executor::new(init::init_systems(tick_time)),
            timed_events: BinaryHeap::new(),
            replicated_entities: HashMap::new(),
            player_inputs: HashMap::new(),
        }
    }

    pub fn get_new_replication_id(&mut self) -> ReplicationId {
        let out = self.replication_counter;
        self.replication_counter += 1;
        ReplicationId(out)
    }
}

impl InputCommand {
    pub fn into_game(
        self,
        replicated_entities: &HashMap<ReplicationId, Entity>,
    ) -> InputCommandGame {
        match self {
            InputCommand::Move(a, b) => InputCommandGame::Move(a, b),
            InputCommand::MoveDelta(a) => InputCommandGame::MoveDelta(a),
            InputCommand::Attack(entity) => {
                if let Some(target) = replicated_entities.get(&ReplicationId(entity)) {
                    InputCommandGame::Attack(*target)
                } else {
                    InputCommandGame::Invalid
                }
            }
            InputCommand::Stop => InputCommandGame::Stop,
            InputCommand::Recall => InputCommandGame::Recall,
            InputCommand::UseAbility(a) => InputCommandGame::UseAbility(a),
            InputCommand::UseAimedAbility(a, b) => InputCommandGame::UseAimedAbility(a, b),
            InputCommand::UseTargetedAbility(a, entity) => {
                if let Some(target) = replicated_entities.get(&ReplicationId(entity)) {
                    InputCommandGame::UseTargetedAbility(a, *target)
                } else {
                    InputCommandGame::Invalid
                }
            }
        }
    }
}
