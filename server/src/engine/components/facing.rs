#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Facing {
    pub direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}
