use crate::ai::get_best_move;
use crate::board::movement::{Move, MoveError, MoveType};
use crate::board::piece::PieceInstance;
use crate::board::piece::side::Side;
use crate::board::position::Position;
use crate::board::{Board, BoardError};
use crate::move_controller::{moves_per_piece, moves_per_side};

#[derive(Clone, Debug)]
pub struct GameManager {
    pub board: Board,
    pub current_turn: Side,
    pub game_over: bool,
    pub winner: Option<Side>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            board: Board::setup(),
            current_turn: Side::Player,
            game_over: false,
            winner: None,
        }
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), GameError> {
        if self.game_over {
            return Err(GameError::GameOver);
        }

        let pos = mv.from;
        let data = self.board.get_square(&pos).ok_or(GameError::InvalidMove)?;
        let piece = PieceInstance {
            position: pos,
            data: *data,
        };
        if piece.data.owner != self.current_turn {
            return Err(GameError::NotYourTurn);
        }

        let possible_moves = moves_per_piece(&self.board, &piece);
        if !possible_moves.contains(&mv) {
            return Err(GameError::InvalidMove);
        }

        self.board.apply_move(&mv)?;

        if mv.move_type == MoveType::Jump {
            let remaining_moves = moves_per_side(&self.board, self.current_turn);
            if remaining_moves
                .iter()
                .any(|m| m.move_type == MoveType::Jump)
            {
                return Ok(());
            }
        }

        self.current_turn = match self.current_turn {
            Side::Player => Side::AI,
            Side::AI => Side::Player,
        };

        let opponent_moves = moves_per_side(&self.board, self.current_turn);
        if opponent_moves.is_empty() {
            self.game_over = true;
            self.winner = Some(match self.current_turn {
                Side::Player => Side::AI,
                Side::AI => Side::Player,
            });
        }

        Ok(())
    }

    pub fn get_possible_moves(&self, pos: Position) -> Vec<Move> {
        if let Some(data) = self.board.get_square(&pos) {
            let piece = PieceInstance {
                position: pos,
                data: *data,
            };
            if piece.data.owner == self.current_turn {
                moves_per_piece(&self.board, &piece)
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    }

    pub fn make_ai_move(&mut self) -> Result<(), GameError> {
        if self.game_over {
            return Err(GameError::GameOver);
        }

        if self.current_turn != Side::AI {
            return Err(GameError::NotAiTurn);
        }

        let best_move = get_best_move(&self.board, Side::AI).ok_or(GameError::NoMovesAvailable)?;

        self.make_move(best_move)
    }
}

#[derive(Debug)]
pub enum GameError {
    GameOver,
    NotYourTurn,
    NotAiTurn,
    InvalidMove,
    NoMovesAvailable,
    BoardError(BoardError),
    MoveError(MoveError),
}

impl From<BoardError> for GameError {
    fn from(e: BoardError) -> Self {
        GameError::BoardError(e)
    }
}

impl From<MoveError> for GameError {
    fn from(e: MoveError) -> Self {
        GameError::MoveError(e)
    }
}
