use nalgebra::base::Vector2;

//TODO: Do we need this component?
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity {
    velocity: Vector2<f32>,
}
