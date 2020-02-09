use std::iter::*;
use std::sync::mpsc::*;
use std::time::Instant;

use legion::prelude::*;
use legion::world::World;
use nalgebra::Vector2;

use super::components::all::*;
use super::game_message::GameMessage;

use super::network::out_message::OutMessage;

pub struct Game {
    receiver: Receiver<GameMessage>,
    tick_time: f32,
    world: World,
    game_time: f32,
    client_out: Sender<OutMessage>
}

impl Game {
    pub fn new(client_out: Sender<OutMessage>, receiver: Receiver<GameMessage>, tick_time: f32) -> Self {
        Self {
            receiver,
            tick_time,
            world: Universe::new().create_world(),
            game_time: 0.,
            client_out,
        }
    }

    pub fn start_loop(&mut self) {
        let mut world = Universe::new().create_world();

        let mut timer = Instant::now();
        let mut accumulator = 0.;

        loop {
            self.check_messages();

            let dt = timer.elapsed();
            let frame_time = dt.as_secs_f32();
            timer = timer + dt;

            accumulator += frame_time;

            if accumulator > self.tick_time {
                while accumulator > self.tick_time {
                    self.update();
                    accumulator -= self.tick_time;
                    self.game_time += self.tick_time;
                }
                self.broadcast_state();
            }
        }
    }

    fn update(&mut self) {
        let mut query = <Write<Transform>>::query();

        for mut transform in query.iter(&mut self.world) {
            transform.position.x = (self.game_time.cos() * 25.) + 100.;
            transform.position.y = (self.game_time.sin() * 25.) + 100.;
        }
    }

    fn check_messages(&mut self) {
        match self.receiver.try_recv() {
            Ok(msg) => self.handle_message(msg),
            Err(e) => self.handle_error(e),
        }
    }

    fn handle_message(&mut self, msg: GameMessage) {
        match msg {
            GameMessage::ClientConnected => {
                println!("Game: Create new player");
                self.on_client_connected();
            }
            GameMessage::MoveCommand { x, y } => println!("Move to x:{}, y:{}", x, y),
        }
    }

    fn handle_error(&self, e: TryRecvError) {
        match e {
            TryRecvError::Empty => (),
            TryRecvError::Disconnected => println!("DISCONNECTED!"),
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

    fn broadcast_state(&self) {
        self.client_out.send(OutMessage::UpdateTick {
            x: 5.,
            y: 10.
        });
    }
}