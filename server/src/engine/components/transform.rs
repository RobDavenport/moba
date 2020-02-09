use nalgebra::Vector2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub position: Vector2<f32>,
    pub scale: Vector2<f32>,
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: Vector2<f32>, scale: Option<Vector2<f32>>, rotation: Option<f32>) -> Self {
        let my_scale = match scale {
            Some(s) => s,
            None => Vector2::new(1., 1.),
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
