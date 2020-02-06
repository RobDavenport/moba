use std::iter::*;
use legion::prelude::*;
use legion::world::World;
use nalgebra::base::Matrix3;

use super::components::all::*;

pub struct Engine {
  world: World
}

impl Engine {
  pub fn new() -> Self {
    let mut world = Universe::new().create_world();

    Self {
      world
    }
  }

  pub fn player_connected(&mut self) {

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