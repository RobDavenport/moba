use nalgebra::base::Matrix3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
  pub transform: Matrix3<f32>
}
