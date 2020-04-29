use legion::prelude::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Attacking {
    pub range: f32,
    pub reload_time: f32,
    pub wind_up_time: f32,
    pub state: AttackingState,
    pub attacking_type: AttackingType,
    pub timer: f32,
    pub target: Option<Entity>,
}

impl Attacking {
    pub fn new(
        range: f32,
        reload_time: f32,
        wind_up_time: f32,
        attacking_type: AttackingType,
    ) -> Self {
        Self {
            range,
            reload_time,
            wind_up_time,
            state: AttackingState::Ready,
            attacking_type,
            timer: 0.,
            target: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum AttackingState {
    Ready,
    WindingUp,
    Cooldown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum AttackingType {
    Instant,
    Projectile, //TODO
}

#[allow(dead_code)]
impl Attacking {
    pub fn attacks_per_second(&self) -> f32 {
        todo!()
    }
}
