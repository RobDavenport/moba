use std::iter::*;
use std::sync::mpsc;
use std::sync::mpsc::*;

use legion::prelude::*;
use legion::world::World;
use nalgebra::base::Matrix3;

use super::components::all::*;
use super::game_message::GameMessage;

pub struct Engine {
    world: World,
    sender: Sender<GameMessage>,
    //receiver: Receiver<GameMessage>,
}

impl Engine {
    pub fn new() -> Self {
        let mut world = Universe::new().create_world();
        let (sender, receiver) = mpsc::channel::<GameMessage>();

        std::thread::spawn(move || {
            loop {
                match receiver.try_recv() {
                    Ok(msg) => {
                        match msg {
                            GameMessage::ClientConnected => {
                                println!("Game: Create new player");
                                //TODO: call on connected / add new player here
                            }
                            GameMessage::MoveCommand { x, y } => {
                                println!("Move to x:{}, y:{}", x, y)
                            }
                        }
                    }
                    Err(e) => (),
                }
            }
        });

        Self {
            world,
            //receiver,
            sender,
        }
    }

    pub fn request_channel(&self) -> Sender<GameMessage> {
        self.sender.clone()
    }

    // pub fn player_connected(&mut self) {
    //   //TODO CALL THIS FUNC
    //   println!("Player connected! - from Engine.rs :)");

    //   self.world.insert(
    //     (),
    //     once((
    //       Transform {
    //         transform: Matrix3::identity()
    //       },
    //       Team {
    //         id: 1
    //       }
    //     ))
    //   );
    // }
}
