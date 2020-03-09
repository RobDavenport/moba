use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MatchId(Uuid);
