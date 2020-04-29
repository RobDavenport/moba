use glam::Vec2;
use legion::prelude::*;

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

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum InputCommandGame {
    Invalid,
    Move(Vec2, bool),
    MoveDelta(Vec2),
    Attack(Entity),
    Stop,
    Recall,
    UseAbility(u8),
    UseAimedAbility(u8, Vec2),
    UseTargetedAbility(u8, Entity),
}
