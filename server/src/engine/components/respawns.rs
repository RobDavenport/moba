#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Respawning {
    pub base_time: f32,
    pub time_remaining: Option<f32>,
}
