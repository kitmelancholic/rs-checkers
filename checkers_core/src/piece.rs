#[derive(Copy, Clone)]
pub struct PieceData {
    pub owner: Side,
    pub is_king: bool,
}

#[derive(Copy, Clone)]
pub struct PieceInstance {
    pub position: Position,
    pub piece: PieceData,
}

#[derive(Copy, Clone)]
pub enum Side {
    Player,
    AI,
}

#[derive(Copy, Clone)]
pub struct Position(pub usize, pub usize);
