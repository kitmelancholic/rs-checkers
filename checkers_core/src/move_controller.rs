use crate::board::Board;
use crate::board::movement::direction::{MoveDirection, MoveHorizontal, MoveVertical};
use crate::board::movement::{Move, MoveType};
use crate::board::piece::PieceInstance;
use crate::board::piece::side::Side;

pub fn moves_per_side(board: &Board, side: Side) -> Vec<Move> {
    let pieces = board.all_pieces_of_side(side);
    let mut moves: Vec<Move> = vec![];

    for piece in pieces {
        moves.extend(all_moves_per_piece(board, &piece));
    }

    // Check if there are jumps: if yes, only allow jumps
    let has_jumps = moves.iter().any(|m| m.move_type == MoveType::Jump);
    if has_jumps {
        moves.retain(|m| m.move_type == MoveType::Jump);
    }

    moves
}

fn all_ways_per_piece(piece: &PieceInstance) -> Vec<MoveDirection> {
    let mut ways: Vec<MoveDirection> = vec![];

    if piece.data.is_king || piece.data.owner == Side::AI {
        ways.push(MoveDirection {
            hor: (MoveHorizontal::Left),
            ver: (MoveVertical::Down),
        });
        ways.push(MoveDirection {
            hor: (MoveHorizontal::Right),
            ver: (MoveVertical::Down),
        });
    }
    if piece.data.is_king || piece.data.owner == Side::Player {
        ways.push(MoveDirection {
            hor: (MoveHorizontal::Left),
            ver: (MoveVertical::Up),
        });
        ways.push(MoveDirection {
            hor: (MoveHorizontal::Right),
            ver: (MoveVertical::Up),
        });
    }
    ways
}

fn all_moves_per_piece(board: &Board, piece: &PieceInstance) -> Vec<Move> {
    let ways = all_ways_per_piece(piece);

    let mut moves: Vec<Move> = vec![];

    for direction in ways {
        if let Some(correct_move) = check_move(board, piece, direction) {
            moves.push(correct_move);
        }
    }
    moves
}

pub fn moves_per_piece(board: &Board, piece: &PieceInstance) -> Vec<Move> {
    let all_side_moves = moves_per_side(board, piece.data.owner);
    all_side_moves
        .into_iter()
        .filter(|m| m.from == piece.position)
        .collect()
}

pub fn check_move(board: &Board, piece: &PieceInstance, move_to: MoveDirection) -> Option<Move> {
    //correct dirrection?
    if !piece.data.is_king
        && match move_to.ver {
            MoveVertical::Up => piece.data.owner != Side::Player,
            MoveVertical::Down => piece.data.owner != Side::AI,
        }
    {
        return None;
    }

    //correct move_to_position?
    match piece.position.setup_moved(&move_to, 1) {
        Ok(moved_pos) => {
            //is there alive piece?
            match board.get_square(&moved_pos) {
                Some(moved_pos_piece) => {
                    //is it in black list?

                    //mine?
                    if moved_pos_piece.owner == piece.data.owner {
                        return None;
                    } else {
                        //is there place behind it?
                        match piece.position.setup_moved(&move_to, 2) {
                            Ok(jump_pos) => {
                                //is there piece?
                                match board.get_square(&jump_pos) {
                                    Some(_) => return None,
                                    None => {
                                        //make_jump
                                        return Some(Move {
                                            from: piece.position,
                                            to: jump_pos,
                                            move_type: MoveType::Jump,
                                        });
                                    }
                                }
                            }
                            Err(_) => return None,
                        }
                    }
                }
                None => {
                    return Some(Move {
                        from: piece.position,
                        to: moved_pos,
                        move_type: MoveType::Move,
                    });
                }
            }
        }
        Err(_) => return None,
    };
}
