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
  receiver: Receiver<GameMessage>,
}

//TODO Loop somewhere and start listening for messages

impl Engine {
  pub fn new() -> Self {
    let mut world = Universe::new().create_world();
    let (sender, receiver) = mpsc::channel::<GameMessage>();

    Self {
      world,
      receiver,
      sender
    }
  }

  pub fn request_channel(&self) -> Sender<GameMessage> {
    self.sender.clone()
  }

  pub fn player_connected(&mut self) {
    //TODO CALL THIS FUNC
    println!("Player connected! - from Engine.rs :)");

    self.world.insert(
      (),
      once((
        Transform {
          transform: Matrix3::identity()
        },
        Team {
          id: 1
        }
      ))
    );
  }
}