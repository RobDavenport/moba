#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Visible {
    pub visibility: VisibilityType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VisibilityType {
    VisibleAll,
    VisibleInRange,
    InvisibleAll,
    // TODO: Need more?
}
