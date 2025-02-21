#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    Move,
    Collision,
}
