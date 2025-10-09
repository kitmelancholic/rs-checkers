pub mod direction;

use super::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub move_type: MoveType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveType {
    Move,
    Jump,
}

#[derive(Debug)]
pub enum MoveError {
    IncorrectStartPosition,
    IncorrectFinishPosition,
    IllegalDirection,
}
