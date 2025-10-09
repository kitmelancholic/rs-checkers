use super::movement::direction::{MoveDirection, MoveHorizontal, MoveVertical};

pub enum PositionError {
    OutOfBounds,
    NotBlack,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn setup(row: isize, col: isize) -> Result<Self, PositionError> {
        if row < 0 || col < 0 || row >= 8 || col >= 8 {
            return Err(PositionError::OutOfBounds);
        }
        if (row + col) % 2 != 1 {
            return Err(PositionError::NotBlack);
        }

        Ok(Position {
            row: row as usize,
            col: col as usize,
        })
    }

    pub fn setup_moved(
        &self,
        dir: &MoveDirection,
        steps: usize,
    ) -> Result<Position, PositionError> {
        let delta_row: isize = match dir.ver {
            MoveVertical::Up => -(steps as isize),
            MoveVertical::Down => steps as isize,
        };

        let delta_col: isize = match dir.hor {
            MoveHorizontal::Left => -(steps as isize),
            MoveHorizontal::Right => steps as isize,
        };

        Self::setup(self.row as isize + delta_row, self.col as isize + delta_col)
    }
}
