#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Team {
    pub id: TeamId,
}

impl Team {
    pub fn new(id: TeamId) -> Self {
        Self { id }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TeamId(pub u32);
