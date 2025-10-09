pub mod board;
pub mod game_manager;
pub mod move_controller;
pub mod movement;
pub mod piece;

pub use board::{Board, BoardError};

pub use piece::{Piece, PiecePosition, Side, calc_next_pos, is_valid_position};

pub use movement::{Move, MoveDirection, MoveError, MoveHorizontal, MoveType, MoveVertical};

pub use move_controller::{all_moves_per_piece, all_moves_per_side, check_move};

pub mod prelude {
    pub use crate::{
        Board, BoardError, Move, MoveDirection, MoveError, MoveHorizontal, MoveType, MoveVertical,
        Piece, PiecePosition, Side, all_moves_per_piece, all_moves_per_side, calc_next_pos,
        check_move, is_valid_position,
    };
}
