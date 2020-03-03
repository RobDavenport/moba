use legion::prelude::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attacking {
    pub range: f32,
    pub reload_time: f32,
    pub wind_up_time: f32,
    pub state: AttackingState,
    pub timer: f32,
    pub target: Option<Entity>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AttackingState {
    Ready,
    WindingUp,
    Cooldown,
}

impl Attacking {
    pub fn attacks_per_second(&self) -> f32 {
        todo!()
    }
}
