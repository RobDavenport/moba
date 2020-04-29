use glam::Vec2;

pub struct Waypoints {
  waypoints: Vec<Vec2>
}

impl Waypoints {
  pub fn new(waypoints: Vec<Vec2>) -> Self {
    Self {
      waypoints
    }
  }
}