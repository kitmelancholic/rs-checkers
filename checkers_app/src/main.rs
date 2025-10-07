use ::checkers_core::{Board, Piece, Side};
use checkers_core::board::BoardError;

fn main() {
    let mut board = Board::setup();

    for row in 0..8 {
        for col in 0..8 {
            let symbol = match board.get_piece(row, col) {
                Ok(piece) => match piece.owner {
                    Side::Player => "P ",
                    Side::AI => "A ",
                },
                Err(BoardError::EmptySquare) => "· ",
                Err(_) => {
                    panic!("Out of bounds");
                }
            };
            print!("{}", symbol);
        }
        print!("\n");
    }
}
