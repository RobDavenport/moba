use nalgebra::Vector2;

#[derive(Debug)]
pub enum InputCommand {
    Move(Vector2<f32>, bool),
    MoveDelta(Vector2<f32>),
    Attack(u32),
    Stop,
    UseAbility(u8),
    UseTargettedAbility(u8, u32),
    UseAimedAbility(u8, Vector2<f32>),
}
