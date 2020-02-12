pub struct StatusEffects {
    effects: Vec<StatusEffect>,
}

pub enum StatusEffect {
    Slow(f32),
    Stun(f32),
}
