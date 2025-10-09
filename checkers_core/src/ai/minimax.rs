use crate::board::Board;
use crate::board::movement::Move;
use crate::board::piece::side::Side;
use crate::move_controller::moves_per_side;

const MAX_DEPTH: usize = 6;

pub fn decide_move(board: &Board, side: Side, depth: usize) -> Option<Move> {
    let moves = moves_per_side(board, side);
    if moves.is_empty() {
        return None;
    }

    let mut best_move = None;
    let mut best_score = i16::MIN;

    for mv in moves {
        let mut board_clone = board.clone();
        if board_clone.apply_move(&mv).is_ok() {
            let score = minimax(&board_clone, depth - 1, i16::MIN, i16::MAX, false, side);
            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }
    }

    best_move
}

fn minimax(
    board: &Board,
    depth: usize,
    mut alpha: i16,
    mut beta: i16,
    is_maximizing: bool,
    ai_side: Side,
) -> i16 {
    if depth == 0 {
        return evaluate_board(board, ai_side);
    }

    let current_side = if is_maximizing {
        Side::AI
    } else {
        Side::Player
    };
    let moves = moves_per_side(board, current_side);

    if moves.is_empty() {
        // No moves available - the game is over
        if is_maximizing {
            return i16::MIN + 1; // AI loses
        } else {
            return i16::MAX - 1; // AI wins
        }
    }

    if is_maximizing {
        let mut max_eval = i16::MIN;
        for mv in moves {
            let mut board_clone = board.clone();
            if board_clone.apply_move(&mv).is_ok() {
                let eval = minimax(&board_clone, depth - 1, alpha, beta, false, ai_side);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break; // Beta cutoff
                }
            }
        }
        max_eval
    } else {
        let mut min_eval = i16::MAX;
        for mv in moves {
            let mut board_clone = board.clone();
            if board_clone.apply_move(&mv).is_ok() {
                let eval = minimax(&board_clone, depth - 1, alpha, beta, true, ai_side);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break; // Alpha cutoff
                }
            }
        }
        min_eval
    }
}

fn evaluate_board(board: &Board, ai_side: Side) -> i16 {
    let ai_pieces = board.all_pieces_of_side(ai_side);
    let opponent_pieces = board.all_pieces_of_side(ai_side.opposite());

    if opponent_pieces.is_empty() {
        return i16::MAX - 1; // AI wins
    }
    if ai_pieces.is_empty() {
        return i16::MIN + 1; // AI loses
    }

    let mut score: i16 = 0;

    // Piece count evaluation
    for piece in &ai_pieces {
        score += if piece.data.is_king { 30 } else { 10 };

        // Position bonus: pieces closer to opponent's side are more valuable
        let row_bonus = match ai_side {
            Side::AI => piece.position.row as i16,
            Side::Player => (7 - piece.position.row) as i16,
        };
        score += row_bonus;
    }

    for piece in &opponent_pieces {
        score -= if piece.data.is_king { 30 } else { 10 };

        let row_bonus = match ai_side.opposite() {
            Side::AI => piece.position.row as i16,
            Side::Player => (7 - piece.position.row) as i16,
        };
        score -= row_bonus;
    }

    // Mobility evaluation: more moves available is better
    let ai_moves = moves_per_side(board, ai_side);
    let opponent_moves = moves_per_side(board, ai_side.opposite());

    score += ai_moves.len() as i16;
    score -= opponent_moves.len() as i16;

    // Bonus for controlling center squares
    for piece in &ai_pieces {
        if (2..6).contains(&piece.position.row) && (2..6).contains(&piece.position.col) {
            score += 2;
        }
    }

    score
}

pub fn get_best_move(board: &Board, side: Side) -> Option<Move> {
    decide_move(board, side, MAX_DEPTH)
}
