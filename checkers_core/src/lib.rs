pub mod ai;
pub mod board;
pub mod game_manager;
pub mod move_controller;

pub use board::{Board, BoardError};

pub use game_manager::{GameError, GameManager};

pub use board::piece::PieceData;
pub use board::piece::side::Side;
pub use board::position::Position;

pub use board::movement::direction::{MoveDirection, MoveHorizontal, MoveVertical};
pub use board::movement::{Move, MoveError, MoveType};

pub use move_controller::{check_move, moves_per_piece, moves_per_side};

pub use ai::{decide_move, get_best_move};

pub mod prelude {
    pub use crate::{
        Board, BoardError, GameError, GameManager, Move, MoveDirection, MoveError, MoveHorizontal,
        MoveType, MoveVertical, PieceData, Position, Side, check_move, decide_move, get_best_move,
        moves_per_piece, moves_per_side,
    };
}
