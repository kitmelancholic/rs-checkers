use crate::Position;
use crate::board::Board;
use crate::piece::Piece;

pub struct Move {
    pub from: Position,
    pub to: MoveDirection,
    pub move_type: MoveType,
}

pub enum MoveType {
    Simple,
    Jump,
    MultiJump<Vec<MoveDirection>>,
    None,
}

pub enum MoveDirection {
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

pub enum MoveError {
    IncorrectPosition,
    WrongDirection,
}

pub fn all_moves_per_piece(board: &Board, piece: &Piece) {
    // let squares = board.get_squares();

    // match piece.owner {

    // }
}

pub fn check_move(
    board: &Board,
    initial_piece: &PieceData,
    from: Position,
    move_to: Position,
    is_multijump_check: bool,
) -> Result<Move, MoveError> {
    let squares = board.get_squares();

    //is there such position? also is it black? -> ok|IncorrectPosition
    //is it possible for this piece?, also if it is king -> ok|WrongDirection

    // is there piece?
    // |-yes, and it in the border?
    // | |-yes -> NoMove
    // | |-no, is it ours?
    // | | |-yes, NoMove
    // | | |-no, is there place behind it?
    // | | | |-yes, is there move available if we get piece there?
    // | | | | |-yes, add to multijump vec -> return piece
    // | | | | |-no -> Jump.
    // | | | |-no -> NoMove.
    // |-no -> SimpleMove
}
