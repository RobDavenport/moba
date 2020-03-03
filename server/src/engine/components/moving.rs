use nalgebra::Vector2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Moving {
    pub base_speed: f32,
    pub location: Option<Vector2<f32>>,
}
