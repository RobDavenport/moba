use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayerControlled {
    pub id: PlayerId,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct PlayerId(pub u32);

impl fmt::Display for PlayerControlled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlayerControlled.id: {}", self.id)
    }
}

impl fmt::Display for PlayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
