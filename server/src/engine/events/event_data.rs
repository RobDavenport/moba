use legion::prelude::*;
use glam::Vec2;

#[derive(Clone, Debug)]
pub struct EventData {
  pub owner: Option<Entity>,
  pub locations: Option<Vec<Vec2>>,
}