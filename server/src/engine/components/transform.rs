use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: Vec2, scale: Option<Vec2>, rotation: Option<f32>) -> Self {
        let my_scale = match scale {
            Some(s) => s,
            None => Vec2::new(1., 1.),
        };

        let my_rotation = match rotation {
            Some(r) => r,
            None => 0.,
        };

        Self {
            position,
            scale: my_scale,
            rotation: my_rotation,
        }
    }
}
