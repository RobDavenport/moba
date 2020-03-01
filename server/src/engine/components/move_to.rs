use nalgebra::Vector2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveTo {
    pub location: Vector2<f32>,
}
