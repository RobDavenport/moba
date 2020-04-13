use glam::Vec2;

use legion::prelude::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Moving {
    pub base_speed: f32,
    pub target: MoveTarget,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveTarget {
    None,
    Location(Vec2),
    Entity(Entity),
}
