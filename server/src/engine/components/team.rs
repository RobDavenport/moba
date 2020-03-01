#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Team {
    pub id: TeamId,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TeamId(u32);
