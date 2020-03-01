#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub health: f32,
    pub max: f32,
    pub regen_rate: f32,
}
