use glam::Vec2;
use legion::prelude::*;

#[derive(Clone, Debug)]
pub struct EventData {
    pub owner: Option<Entity>,
    pub locations: Option<Vec<Vec2>>,
}
