use std::collections::{HashMap, VecDeque};
use std::iter::*;
use std::time::{Duration, Instant};

use legion::{prelude::*, world::World};
use nalgebra::Vector2;
use tokio::sync::mpsc::{Receiver, Sender};

use super::components::all::*;
use super::game_events::GameEvent;
use super::input_command::InputCommand;
use super::systems::*;
use crate::engine::messaging::messages::{EntitySnapshot, GameMessage, OutMessage, OutTarget};

const SNAPSHOT_HISTORY_MAX_SIZE: usize = 64;

pub struct Game {
    tick_time: f32,
    world: World,
    game_time: f32,
    game_frame: u32,
    out_reliable: Sender<(OutTarget, OutMessage)>,
    out_unreliable: Sender<(OutTarget, OutMessage)>,
    game_in: Receiver<GameMessage>,
    player_entities: HashMap<PlayerId, Entity>,
    player_snapshot_history: HashMap<PlayerId, SnapshotHistory>,
    replication_counter: u32,
    game_events: VecDeque<GameEvent>,
}

struct SnapshotHistory {
    history: VecDeque<SnapshotData>,
    ack_baseline: Option<SnapshotData>,
}

struct SnapshotData {
    pub entity_data: Vec<EntitySnapshot>,
    pub frame: u32,
}

impl SnapshotHistory {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(SNAPSHOT_HISTORY_MAX_SIZE),
            ack_baseline: None,
        }
    }

    pub fn save_and_calc_delta(
        &mut self,
        in_snapshot: SnapshotData,
    ) -> Option<Vec<EntitySnapshot>> {
        if self.history.len() == SNAPSHOT_HISTORY_MAX_SIZE {
            self.history.clear();
            return None;
        }

        if let Some(baseline) = &self.ack_baseline {
            //calcualte the deltas, add them to out, and send
            //let mut out = Vec::new();
            //TODO fix this
            Some(baseline.entity_data.clone())
        } else {
            self.history.push_back(in_snapshot);
            None
        }
    }

    pub fn set_new_baseline(&mut self, frame: u32) {
        self.history.retain(|snap| snap.frame >= frame); //only keep "newer snapsots"
        self.ack_baseline = self.history.pop_front();
    }
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
            player_snapshot_history: HashMap::new(),
        }
    }

    pub async fn start_game(&mut self) {
        let mut timer = Instant::now();
        let mut accumulator = 0.;
        let mut game_running = true;

        println!("GAME LOOP INITIATED");

        let mut executor = Executor::new(init_systems(self.tick_time));
        let mut ticker = tokio::time::interval(Duration::from_secs_f32(self.tick_time));

        while game_running {
            let dt = timer.elapsed();
            let frame_time = dt.as_secs_f32();
            timer = timer + dt;

            accumulator += frame_time;

            while let Ok(game_message) = self.game_in.try_recv() {
                self.handle_message(game_message);
            }

            if accumulator > self.tick_time {
                while accumulator > self.tick_time {
                    self.game_frame += 1;
                    self.game_time += self.tick_time;
                    executor.execute(&mut self.world);
                    accumulator -= self.tick_time;
                }

                self.broadcast_state().await;
            }

            ticker.tick().await;
        }
    }

    fn handle_message(&mut self, msg: GameMessage) {
        match msg {
            GameMessage::ClientConnected(id) => {
                println!("Game: Create new player");
                self.on_client_connected(id);
            }
            GameMessage::InputCommand { id, command } => self.handle_input_command(id, command),
            GameMessage::ClientDisconnected(id) => {
                println!("Game: Client disconnected");
                self.on_client_disconnected(id)
            }
            GameMessage::Ack { id, new_baseline } => self.handle_ack(id, new_baseline),
        }
    }

    fn handle_ack(&mut self, id: PlayerId, new_baseline: u32) {
        //TODO!
    }

    fn handle_input_command(&mut self, id: PlayerId, command: InputCommand) {
        match command {
            InputCommand::Move(loc, _attacking) => {
                //todo: change to pawn stuff
                if let Some(mut moving) = self
                    .world
                    .get_component_mut::<Moving>(*self.player_entities.get(&id).unwrap())
                {
                    moving.target = MoveTarget::Location(loc);
                    println!("player {} moved to: {}", id, loc);
                }
            }
            _ => println!("Unhaled Input Command!"),
        }
    }

    fn on_client_connected(&mut self, player_id: PlayerId) {
        let replication_id = self.get_new_replication_id();
        let entities = self.world.insert(
            (),
            once((
                Transform::new(Vector2::<f32>::new(1., 1.), None, None),
                Replicated {
                    id: ReplicationId(replication_id),
                },
                PlayerControlled { id: player_id },
                Moving {
                    base_speed: 125.,
                    target: MoveTarget::None,
                },
            )),
        );

        let player_entity = entities.first().unwrap();

        self.player_entities.insert(player_id, *player_entity);
    }

    fn on_client_disconnected(&mut self, player_id: PlayerId) {
        if let Some(to_remove) = self.player_entities.remove(&player_id) {
            if let Some(replicated) = self.world.get_component_mut::<Replicated>(to_remove) {
                self.game_events
                    .push_back(GameEvent::EntityDestroyed(replicated.id));
            }
            if self.world.delete(to_remove) {
                self.game_events
                    .push_back(GameEvent::ClientDisconnected(player_id));
            }
        }
    }

    async fn broadcast_state(&mut self) {
        //Todo only send 'dirty' components
        let query = <(Read<Transform>, Read<Replicated>)>::query();

        let entities: Vec<EntitySnapshot> = query
            .iter(&mut self.world)
            .map(|(transform, replicated)| EntitySnapshot {
                x: transform.position.x.into(),
                y: transform.position.y.into(),
                replication_id: replicated.id,
            })
            .collect();

        if entities.len() > 0 {
            let snapshot = OutMessage::Snapshot {
                frame: self.game_frame,
                entities,
            };

            self.out_unreliable.try_send((OutTarget::All, snapshot));
        }

        for event in self.game_events.drain(..) {
            match event {
                GameEvent::EntityDestroyed(id) => self.out_reliable.try_send((
                    OutTarget::All,
                    OutMessage::EntityDestroyed {
                        frame: self.game_frame,
                        replication_id: id,
                    },
                )),
                GameEvent::ClientDisconnected(_) => Ok(()),
            };
        }
    }

    fn get_new_replication_id(&mut self) -> u32 {
        let out = self.replication_counter;
        self.replication_counter += 1;
        out
    }
}

fn init_systems(tick_time: f32) -> Vec<Box<dyn Schedulable>> {
    let mut out = Vec::new();

    println!("Initialized game systems with tick time of {}s", tick_time);

    out.push(pawn_move::pawn_move(tick_time));

    out
}
