#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Visible {
    pub visibility: VisibilityType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum VisibilityType {
    VisibleAll,
    VisibleInRange,
    InvisibleAll,
    // TODO: Need more?
}
