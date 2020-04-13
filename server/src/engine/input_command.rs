use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputCommand {
    Move(Vec2, bool),
    MoveDelta(Vec2),
    Attack(u32),
    Stop,
    Recall,
    UseAbility(u8),
    UseTargettedAbility(u8, u32),
    UseAimedAbility(u8, Vec2),
}
