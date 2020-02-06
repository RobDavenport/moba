use nalgebra::base::Matrix3;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Transform {
  transform: Matrix3<f32>
}