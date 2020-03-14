#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Replicated {
    pub id: ReplicationId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReplicationId(pub u32);
