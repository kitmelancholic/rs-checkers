use crate::board::Board;
use crate::movement::{Move, MoveDirection, MoveError, MoveHorizontal, MoveType, MoveVertical};
use crate::piece::{Piece, Side, calc_next_pos, is_valid_position};

pub fn all_moves_per_side(board: &Board, side: Side) -> Vec<Move> {
    let pieces = board.all_pieces_of_side(side);
    let mut moves: Vec<Move> = vec![];

    for piece in pieces {
        moves.extend(all_moves_per_piece(board, piece));
    }

    moves
}

pub fn all_moves_per_piece(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut ways: Vec<MoveDirection> = vec![];

    if piece.is_king || piece.owner == Side::AI {
        ways.push(MoveDirection {
            hor: (MoveHorizontal::Left),
            ver: (MoveVertical::Down),
        });
        ways.push(MoveDirection {
            hor: (MoveHorizontal::Right),
            ver: (MoveVertical::Down),
        });
    }
    if piece.is_king || piece.owner == Side::Player {
        ways.push(MoveDirection {
            hor: (MoveHorizontal::Left),
            ver: (MoveVertical::Up),
        });
        ways.push(MoveDirection {
            hor: (MoveHorizontal::Right),
            ver: (MoveVertical::Up),
        });
    }

    let mut moves: Vec<Move> = vec![];

    for direction in ways {
        if let Ok(correct_move) = check_move(board, piece, direction) {
            moves.push(correct_move);
        }
    }
    moves
}

pub fn check_move(board: &Board, piece: &Piece, move_to: MoveDirection) -> Result<Move, MoveError> {
    let from_row = piece.position.row as isize;
    let from_col = piece.position.col as isize;

    if !is_valid_position(from_row, from_col) {
        return Err(MoveError::IncorrectPosition);
    } else if !piece.is_king
        && match move_to.ver {
            MoveVertical::Up => piece.owner != Side::Player,
            MoveVertical::Down => piece.owner != Side::AI,
        }
    {
        return Err(MoveError::WrongDirection);
    } else {
        match calc_next_pos(&piece.position, &move_to) {
            Ok(next_pos) => match board.is_empty(next_pos.row, next_pos.col) {
                Ok(true) => {
                    return Ok(Move {
                        from: piece.position,
                        to: next_pos,
                        move_type: MoveType::Simple,
                    });
                }
                Ok(false) => match calc_next_pos(&next_pos, &move_to) {
                    Ok(jump_pos) => match board.is_empty(jump_pos.row, jump_pos.col) {
                        Ok(true) => {
                            return Ok(Move {
                                from: piece.position,
                                to: jump_pos,
                                move_type: MoveType::Jump,
                            });
                        }
                        Ok(false) => {
                            return Err(MoveError::NoMove);
                        }
                        Err(_) => panic!(),
                    },
                    Err(MoveError::IllegalMove) => {
                        return Err(MoveError::NoMove);
                    }
                    Err(_) => panic!(),
                },
                Err(_) => panic!(),
            },
            Err(MoveError::IllegalMove) => {
                return Err(MoveError::IllegalMove);
            }
            _ => panic!(),
        }
    }

    // is there piece? +
    // |-yes, and it in the border?
    // | |-yes -> NoMove
    // | |-no, is it ours?
    // | | |-yes, NoMove
    // | | |-no, is there place behind it?
    // | | | |-yes -> Jump
    // | | | |-no -> NoMove.
    // |-no -> SimpleMove +
}
