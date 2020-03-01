#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveAbility {
    pub ability_id: AbilityId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AbilityId(u32);
