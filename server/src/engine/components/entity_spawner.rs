#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntitySpawner {
    pub spawn_style: SpawnStyle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpawnStyle {
    DelayedOnce(f32),
    DelayRepeated(f32),
}
