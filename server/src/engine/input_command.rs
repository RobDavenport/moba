use glam::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum InputCommand {
    Move(Vec2, bool),
    MoveDelta(Vec2),
    Attack(u32),
    Stop,
    Recall,
    UseAbility(u8),
    UseAimedAbility(u8, Vec2),
    UseTargetedAbility(u8, u32),
}
