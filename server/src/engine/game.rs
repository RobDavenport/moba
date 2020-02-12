use std::iter::*;
use std::sync::mpsc::*;
use std::time::Instant;

use legion::prelude::*;
use legion::world::World;
use nalgebra::Vector2;

use super::components::all::*;
use super::input_command::InputCommand;
use crate::engine::messaging::message_listener::MessageListener;
use crate::engine::messaging::messages::{GameMessage, OutMessage};

const SLEEP_NANO_SECONDS: u64 = 1;

pub struct Game {
    tick_time: f32,
    world: World,
    game_time: f32,
    game_frame: u32,
    client_out: Sender<OutMessage>,
    game_message_listener: MessageListener<GameMessage>,
}

impl Game {
    pub fn new(
        client_out: Sender<OutMessage>,
        receiver: Receiver<GameMessage>,
        tick_time: f32,
    ) -> Self {
        Self {
            tick_time,
            world: Universe::new().create_world(),
            game_time: 0.,
            client_out,
            game_frame: 0,
            game_message_listener: MessageListener::new(receiver),
        }
    }

    pub fn start_loop(&mut self) {
        let sleep_duration = std::time::Duration::from_nanos(SLEEP_NANO_SECONDS);

        let mut timer = Instant::now();
        let mut accumulator = 0.;

        loop {
            if let Some(mut game_messages) = self.game_message_listener.check_messages() {
                for msg in game_messages.drain(..) {
                    self.handle_message(msg)
                }
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
                self.broadcast_state();
            }

            std::thread::sleep(sleep_duration);
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
    }

    fn broadcast_state(&mut self) {
        let query = <Read<Transform>>::query();

        //Todo only send 'dirty' components
        for transform in query.iter(&mut self.world) {
            self.client_out
                .send(OutMessage::UpdateTick {
                    f: self.game_frame,
                    x: transform.position.x,
                    y: transform.position.y,
                })
                .unwrap();
        }
    }
}
