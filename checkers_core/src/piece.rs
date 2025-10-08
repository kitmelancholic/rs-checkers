use crate::movement::{MoveDirection, MoveError, MoveHorizontal, MoveVertical};

#[derive(Copy, Clone)]
pub struct Piece {
    pub position: PiecePosition,
    pub owner: Side,
    pub is_king: bool,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Side {
    Player,
    AI,
}

#[derive(Copy, Clone)]
pub struct PiecePosition {
    pub row: usize,
    pub col: usize,
}

pub fn is_valid_position(row: isize, col: isize) -> bool {
    row >= 0 && col >= 0 && row < 8 && col < 8 && (row + col) % 2 == 1
}

pub fn calc_next_pos(pos: &PiecePosition, dir: &MoveDirection) -> Result<PiecePosition, MoveError> {
    let delta_row: isize = match dir.ver {
        MoveVertical::Up => -1,
        MoveVertical::Down => 1,
    };

    let delta_col: isize = match dir.hor {
        MoveHorizontal::Left => -1,
        MoveHorizontal::Right => 1,
    };

    if is_valid_position(pos.row as isize + delta_row, pos.col as isize + delta_col) {
        Ok(PiecePosition {
            row: (pos.row as isize + delta_row) as usize,
            col: (pos.col as isize + delta_col) as usize,
        })
    } else {
        Err(MoveError::IllegalMove)
    }
}
