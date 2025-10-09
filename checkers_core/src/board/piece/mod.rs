pub mod side;

use crate::Position;

use self::side::Side;

#[derive(Copy, Clone, Debug)]
pub struct PieceData {
    pub owner: Side,
    pub is_king: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct PieceInstance {
    pub position: Position,
    pub data: PieceData,
}
