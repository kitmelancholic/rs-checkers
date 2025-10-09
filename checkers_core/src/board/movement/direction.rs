pub struct MoveDirection {
    pub hor: MoveHorizontal,
    pub ver: MoveVertical,
}

pub enum MoveHorizontal {
    Left,
    Right,
}

pub enum MoveVertical {
    Up,
    Down,
}
