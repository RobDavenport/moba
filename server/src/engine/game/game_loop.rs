use std::collections::binary_heap::PeekMut;
use std::iter::*;
use std::time::{Duration, Instant};

use legion::prelude::*;

use super::Game;
use crate::engine::components::all::*;
use crate::engine::events::{game_event::GameEvent, timed_event::TimedEventType};
use crate::engine::input_command::InputCommand;
use crate::engine::messaging::messages::{EntitySnapshot, GameMessage, OutMessage, OutTarget};
use crate::engine::network::delta_encoder::SnapshotHistory;

impl Game {
    pub async fn start_game_loop(&mut self) {
        let mut timer = Instant::now();
        let mut accumulator = 0.;
        let game_running = true;

        let mut ticker = tokio::time::interval(Duration::from_secs_f32(self.tick_time));

        println!("GAME LOOP INITIATED");

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
                    self.executor.execute(&mut self.world);
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
        if let Some(history) = self.player_snapshot_histories.get_mut(&id) {
            history.ack_baseline(new_baseline);
        }
    }

    fn handle_input_command(&mut self, id: PlayerId, command: InputCommand) {
        if let Some(mut recieve_input) = self
            .world
            .get_component_mut::<ReceiveInput>(*self.player_entities.get(&id).unwrap())
        {
            recieve_input.next_command = Some(command)
        };
    }

    fn on_client_connected(&mut self, player_id: PlayerId) {
        let player_entity = self.insert_player(player_id);

        self.player_entities.insert(player_id, player_entity);
        self.player_snapshot_histories
            .insert(player_id, SnapshotHistory::new());
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
        self.player_snapshot_histories.remove(&player_id);
    }

    async fn broadcast_state(&mut self) {
        let query = <(Read<Transform>, Read<Replicated>)>::query();

        let mut entities: Vec<EntitySnapshot> = query
            .iter(&mut self.world)
            .map(|(transform, replicated)| EntitySnapshot {
                x: Some(transform.position.x().into()),
                y: Some(transform.position.y().into()),
                rotation: Some(transform.rotation.into()),
                replication_id: replicated.id,
            })
            .collect();

        if entities.len() > 0 {
            entities.sort_unstable();

            for (id, history) in self.player_snapshot_histories.iter_mut() {
                if let Some((baseline, delta_entities)) =
                    history.encode_delta(self.game_frame, &entities)
                {
                    if delta_entities.len() > 0 {
                        //println!("d {} => {}", baseline, self.game_frame);
                        if let Err(e) = self.out_unreliable.try_send((
                            OutTarget::Single(*id),
                            OutMessage::Snapshot {
                                frame: self.game_frame,
                                entities: delta_entities,
                                baseline: Some(baseline),
                            },
                        )) {
                            println!("Error in Broadcast State: {}", &e);
                        };
                    }
                } else {
                    if let Err(e) = self.out_unreliable.try_send((
                        OutTarget::Single(*id),
                        OutMessage::Snapshot {
                            frame: self.game_frame,
                            entities: entities.clone(),
                            baseline: None,
                        },
                    )) {
                        println!("Error in Broadcast State: {}", &e);
                    };
                    println!("Full Snapshot");
                }
            }
        }

        self.handle_events();
    }

    fn handle_events(&mut self) {
        // Priority Queue
        let mut next_events = Vec::new();
        while let Some(event) = self.timed_events.peek_mut() {
            if event.execute_frame <= self.game_frame {
                match event.event_type {
                    TimedEventType::Repeating(offset) => {
                        next_events.push(event.new_repeated(offset))
                    }
                    TimedEventType::Once => (),
                }
                (event.execute)(&mut self.world);
                PeekMut::pop(event);
            } else {
                break;
            }
        }

        for event in next_events.into_iter() {
            self.timed_events.push(event)
        }

        // Normal "Game Events" Vec
        for event in self.game_events.drain(..) {
            if let Err(e) = match event {
                GameEvent::EntityDestroyed(id) => self.out_reliable.try_send((
                    OutTarget::All,
                    OutMessage::EntityDestroyed {
                        frame: self.game_frame,
                        replication_id: id,
                    },
                )),
                GameEvent::ClientDisconnected(_) => Ok(()),
            } {
                println!("Error in game_events: {}", &e);
            }
        }
    }
}
