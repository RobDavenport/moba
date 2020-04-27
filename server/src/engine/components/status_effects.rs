#[allow(dead_code)]
pub struct StatusEffects {
    effects: Vec<StatusEffect>,
}

#[allow(dead_code)]
pub enum StatusEffect {
    MoveSpeed(f32),
    AttackSpeed(f32),
    DamageMod(f32),
    Stun(f32),
    Suppression(f32),
}
