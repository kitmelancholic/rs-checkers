pub mod movement;
pub mod piece;
pub mod position;

use crate::MoveError;
use crate::board::piece::PieceInstance;

use self::movement::{Move, MoveType};
use self::piece::PieceData;
use self::piece::side::Side;
use self::position::Position;

#[derive(Debug)]
pub enum BoardError {
    OutOfBounds,
    EmptySquare,
}

#[derive(Clone, Debug)]
pub struct Board {
    squares: [[Option<PieceData>; 8]; 8],
}

impl Board {
    pub fn setup() -> Self {
        let mut board = Board {
            squares: [[None; 8]; 8],
        };

        for row in 0..3 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    board.squares[row][col] = Some(PieceData {
                        owner: Side::AI,
                        is_king: false,
                    });
                }
            }
        }

        for row in 5..8 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    board.squares[row][col] = Some(PieceData {
                        owner: Side::Player,
                        is_king: false,
                    });
                }
            }
        }

        board
    }

    pub fn get_square(&self, pos: &Position) -> Option<&PieceData> {
        self.squares[pos.row][pos.col].as_ref()
    }

    pub fn all_pieces_of_side(&self, side: Side) -> Vec<PieceInstance> {
        let mut instances = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if let Some(data) = &self.squares[row][col] {
                    if data.owner == side {
                        instances.push(PieceInstance {
                            position: Position { row, col },
                            data: *data,
                        });
                    }
                }
            }
        }
        instances
    }

    pub fn apply_move(&mut self, mv: &Move) -> Result<(), MoveError> {
        let piece = match self.squares[mv.from.row][mv.from.col].take() {
            Some(p) => p,
            None => return Err(MoveError::IncorrectStartPosition),
        };

        let piece = PieceData {
            owner: piece.owner,
            is_king: piece.is_king
                || (mv.to.row == 0 && piece.owner == Side::Player)
                || (mv.to.row == 7 && piece.owner == Side::AI),
        };

        self.squares[mv.to.row][mv.to.col] = Some(piece);

        // Only remove the captured piece if this is a jump move
        if mv.move_type == MoveType::Jump {
            let take_row = (mv.from.row + mv.to.row) / 2;
            let take_col = (mv.from.col + mv.to.col) / 2;
            self.squares[take_row][take_col] = None;
        }

        Ok(())
    }

    pub fn score_of_side(&self, side: Side) -> i16 {
        (self.all_pieces_of_side(side).len() as i16 * 3)
            - (self.all_pieces_of_side(side.opposite()).len() as i16)
    }
}
