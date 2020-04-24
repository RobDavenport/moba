use legion::prelude::*;
use legion::world::World;

// pub mod tower;
// pub mod minion;
// pub mod core;
// pub mod spawner;
// pub mod character;

pub mod all {
    // pub use super::tower::*;
    // pub use super::minion::*;
    // pub use super::core::*;
    // pub use super::spawner::*;
    // pub use super::character::*;
}

trait Archetype {
    fn insert_single_from_data<T>(world: &World, data: &T) -> Entity;
    fn insert_many_from_data<T>(world: &World, data: &Vec<T>) -> Vec<Entity>;
}