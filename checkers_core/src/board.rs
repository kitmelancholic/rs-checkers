use crate::piece::{Piece, Position, Side};

pub enum BoardError {
    OutOfBounds,
    EmptySquare,
}

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn setup() -> Self {
        let mut board = Board {
            squares: [[None; 8]; 8],
        };

        for row in 0..3 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    board.squares[row][col] = Some(Piece {
                        position: Position(row, col),
                        owner: Side::AI,
                        is_king: false,
                    });
                }
            }
        }

        for row in 5..8 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    board.squares[row][col] = Some(Piece {
                        position: Position(row, col),
                        owner: Side::Player,
                        is_king: false,
                    });
                }
            }
        }

        board
    }

    pub fn get_piece(&self, row: usize, col: usize) -> Result<&Piece, BoardError> {
        if row >= 8 || col >= 8 {
            return Err(BoardError::OutOfBounds);
        }

        match self.squares[row as usize][col as usize].as_ref() {
            Some(piece) => Ok(piece),
            None => Err(BoardError::EmptySquare),
        }
    }

    pub fn is_empty(&self, row: usize, col: usize) -> Result<bool, BoardError> {
        match self.get_piece(row, col) {
            Ok(_) => Ok(false),
            Err(BoardError::EmptySquare) => Ok(true),
            Err(e) => Err(e),
        }
    }

    pub fn get_squares(&self) -> &[[Option<Piece>; 8]; 8] {
        &self.squares
    }
}
