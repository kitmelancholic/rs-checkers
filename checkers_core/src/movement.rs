use crate::piece::PiecePosition;

pub struct Move {
    pub from: PiecePosition,
    pub to: PiecePosition,
    pub move_type: MoveType,
}

pub enum MoveType {
    Simple,
    Jump,
    None,
}

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

pub enum MoveError {
    IncorrectPosition,
    WrongDirection,
    IllegalMove,
    NoMove,
}
