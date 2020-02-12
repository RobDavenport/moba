use nalgebra::Vector2;

pub enum InputCommand {
    Move(Vector2<f32>),
    AttackMove(Vector2<f32>),
    Attack(u32),
    Stop,
    UseAbility(u8),
    UseTargettedAbility(u32),
    UseAimedAbility(Vector2<f32>),
}
