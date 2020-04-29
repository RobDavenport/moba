use glam::Vec2;

#[allow(dead_code)]
pub struct Waypoints {
    waypoints: Vec<Vec2>,
}

impl Waypoints {
    pub fn new(waypoints: Vec<Vec2>) -> Self {
        Self { waypoints }
    }
}
