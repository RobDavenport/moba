use std::iter::*;
use std::time::Duration;
use std::time::Instant;

use futures::join;

use legion::prelude::*;
use legion::world::World;
use nalgebra::Vector2;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::delay_for;

use super::components::all::*;
use super::input_command::InputCommand;
use crate::engine::messaging::messages::{GameMessage, OutMessage};

pub struct Game {
    tick_time: f32,
    world: World,
    game_time: f32,
    game_frame: u32,
    client_out_reliable: Sender<OutMessage>,
    client_out_unreliable: Sender<OutMessage>,
    game_message_listener_reliable: Receiver<GameMessage>,
    game_message_listener_unreliable: Receiver<GameMessage>,
}

impl Game {
    pub fn new(
        tick_time: f32,
        client_out_reliable: Sender<OutMessage>,
        client_out_unreliable: Sender<OutMessage>,
        receiver_reliable: Receiver<GameMessage>,
        receiver_unreliable: Receiver<GameMessage>,
    ) -> Self {
        Self {
            tick_time,
            world: Universe::new().create_world(),
            game_time: 0.,
            game_frame: 0,
            client_out_reliable,
            client_out_unreliable,
            game_message_listener_reliable: receiver_reliable,
            game_message_listener_unreliable: receiver_unreliable,
        }
    }

    pub async fn start_game(&mut self) {
        let mut timer = Instant::now();
        let mut accumulator = 0.;
        let mut updated: bool;

        println!("GAME LOOP INITIATED");

        loop {
            if let Ok(game_message) = self.game_message_listener_reliable.try_recv() {
                self.handle_message(game_message);
            }

            if let Ok(game_message) = self.game_message_listener_unreliable.try_recv() {
                self.handle_message(game_message);
            }

            let dt = timer.elapsed();
            let frame_time = dt.as_secs_f32();
            timer = timer + dt;

            accumulator += frame_time;

            if accumulator > self.tick_time {
                while accumulator > self.tick_time {
                    self.game_frame += 1;
                    self.game_time += self.tick_time;
                    self.update(self.tick_time);
                    accumulator -= self.tick_time;
                }
                updated = true;
            } else {
                updated = false;
            }

            if updated {
                let time = self.tick_time;
                join!(
                    self.broadcast_state(),
                    delay_for(Duration::from_secs_f32(time))
                );
            } else {
                delay_for(Duration::from_secs_f32(self.tick_time)).await;
            }
        }
    }

    fn update(&mut self, _dt: f32) {
        let query = <Write<Transform>>::query();

        for mut transform in query.iter(&mut self.world) {
            transform.position.x = (self.game_time.cos() * 50.) + 125.;
            transform.position.y = (self.game_time.sin() * 50.) + 125.;
        }
    }

    fn handle_message(&mut self, msg: GameMessage) {
        match msg {
            GameMessage::ClientConnected => {
                println!("Game: Create new player");
                self.on_client_connected();
            }
            GameMessage::InputCommand { id, command } => self.handle_input_command(id, command),
            _ => panic!("Unhandled GameMessage!"),
        }
    }

    fn handle_input_command(&mut self, id: u32, command: InputCommand) {
        match command {
            InputCommand::Move(target) => println!("User-{}: Move to: {}", id, target),
            _ => panic!("Unhaled Input Command!"),
        }
    }

    fn on_client_connected(&mut self) {
        self.world.insert(
            (),
            once((
                Transform::new(Vector2::<f32>::new(1., 1.), None, None),
                Team { id: 1 },
            )),
        );

        self.world.insert(
            (),
            once((
                Transform::new(Vector2::<f32>::new(1., 1.), None, None),
                Team { id: 2 },
            )),
        );
    }

    async fn broadcast_state(&mut self) {
        let query = <(Read<Transform>, Read<Team>)>::query();

        //Todo only send 'dirty' components
        for (transform, team) in query.iter(&mut self.world) {
            let output = OutMessage::UpdateTick {
                f: self.game_frame,
                x: transform.position.x + (100. * team.id as f32),
                y: transform.position.y,
                n: team.id as u32,
            };

            if (team.id == 1) {
                self.client_out_reliable.send(output).await;
            } else if (team.id == 2) {
                self.client_out_unreliable.send(output).await;
            }

            // let f1 = self.client_out_reliable.send(output);
            // let f2 = self.client_out_unreliable.send(output);
            //join!(f1, f2);
        }
    }
}
