use checkers_core::prelude::*;
use iced::{
    Color, Element, Length, Point, Rectangle, Size, Task, event, mouse,
    widget::{
        Canvas,
        canvas::{self, Frame, Geometry, Path, Stroke, Text},
        column, container, text,
    },
};

pub fn main() -> iced::Result {
    iced::run("Checkers - rs-checkers", update, view)
}

fn update(state: &mut CheckersUI, message: Message) -> Task<Message> {
    state.update(message)
}

fn view(state: &CheckersUI) -> Element<'_, Message> {
    state.view()
}

#[derive(Debug, Clone)]
pub struct CheckersUI {
    game: GameManager,
    selected_piece: Option<Position>, // Track the selected piece
    available_moves: Vec<Move>,       // Highlight available moves
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CellClicked(usize, usize),
    AiMove,
}

#[derive(Debug)]
struct BoardCanvas {
    game: GameManager,
    selected_piece: Option<Position>,
    available_moves: Vec<Move>,
}

impl BoardCanvas {
    fn new(game: &GameManager, selected: Option<Position>, available: &[Move]) -> Self {
        Self {
            game: game.clone(),
            selected_piece: selected,
            available_moves: available.to_vec(),
        }
    }
}

impl canvas::Program<Message> for BoardCanvas {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        if let canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
            let cell_size = bounds.width / 8.0;
            let rel_pos = cursor.position().unwrap_or(Point::new(0.0, 0.0)) - bounds.position();
            let col = (rel_pos.x / cell_size) as usize;
            let row = (rel_pos.y / cell_size) as usize;
            if row < 8 && col < 8 {
                return (
                    event::Status::Captured,
                    Some(Message::CellClicked(row, col)),
                );
            }
        }
        (event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let cell_size = bounds.width / 8.0;

        for row in 0..8 {
            for col in 0..8 {
                let x = col as f32 * cell_size;
                let y = row as f32 * cell_size;
                let cell_color = if (row + col) % 2 == 0 {
                    Color::from_rgb(0.93, 0.93, 0.82)
                } else {
                    Color::from_rgb(0.46, 0.59, 0.34)
                };
                frame.fill_rectangle(
                    Point::new(x, y),
                    Size::new(cell_size, cell_size),
                    cell_color,
                );

                if let Some(piece) = self.game.board.get_square(&Position { row, col }) {
                    let center = Point::new(x + cell_size / 2.0, y + cell_size / 2.0);
                    let radius = cell_size / 2.0 * 0.8;
                    let bg_color = if piece.owner == Side::Player {
                        Color::from_rgb(1.0, 0.0, 0.0)
                    } else {
                        Color::from_rgb(0.0, 0.0, 1.0)
                    };
                    frame.fill(&Path::circle(center, radius), bg_color);

                    if piece.is_king {
                        let text = Text {
                            content: "K".to_string(),
                            position: center,
                            color: Color::WHITE,
                            size: iced::Pixels(20.0),
                            horizontal_alignment: iced::alignment::Horizontal::Center,
                            vertical_alignment: iced::alignment::Vertical::Center,
                            ..Default::default()
                        };
                        frame.fill_text(text);
                    }
                }

                if self.selected_piece == Some(Position { row, col }) {
                    let stroke = Stroke {
                        style: canvas::Style::Solid(Color::from_rgb(1.0, 0.84, 0.0)),
                        width: 3.0,
                        ..Default::default()
                    };
                    frame.stroke(
                        &Path::rectangle(Point::new(x, y), Size::new(cell_size, cell_size)),
                        stroke,
                    );
                }

                if self
                    .available_moves
                    .iter()
                    .any(|m| m.to == Position { row, col })
                {
                    let dot_center = Point::new(x + cell_size / 2.0, y + cell_size / 2.0);
                    frame.fill(
                        &Path::circle(dot_center, 12.0),
                        Color::from_rgb(0.0, 1.0, 0.0),
                    );
                }
            }
        }

        vec![frame.into_geometry()]
    }
}

impl CheckersUI {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CellClicked(row, col) => {
                // Don't allow clicking if it's AI's turn
                if self.game.current_turn == Side::AI {
                    return Task::none();
                }

                let pos = Position { row, col };
                if self.selected_piece.is_some() {
                    if let Some(mv) = self.available_moves.iter().find(|m| m.to == pos).cloned() {
                        let _ = self.game.make_move(mv);
                        self.selected_piece = None;
                        self.available_moves.clear();

                        // After player move, trigger AI move if it's AI's turn with delay
                        if self.game.current_turn == Side::AI && !self.game.game_over {
                            return Task::perform(
                                async {
                                    async_std::task::sleep(std::time::Duration::from_millis(800))
                                        .await;
                                },
                                |_| Message::AiMove,
                            );
                        }
                    } else {
                        self.selected_piece = None;
                        self.available_moves.clear();
                    }
                } else {
                    if let Some(piece) = self.game.board.get_square(&pos) {
                        if piece.owner == self.game.current_turn {
                            self.selected_piece = Some(pos);
                            self.available_moves = self.game.get_possible_moves(pos);
                        }
                    }
                }
                Task::none()
            }
            Message::AiMove => {
                if self.game.current_turn == Side::AI && !self.game.game_over {
                    let _ = self.game.make_ai_move();

                    // If still AI's turn (multiple jumps), schedule another AI move with shorter delay
                    if self.game.current_turn == Side::AI && !self.game.game_over {
                        return Task::perform(
                            async {
                                async_std::task::sleep(std::time::Duration::from_millis(500)).await;
                            },
                            |_| Message::AiMove,
                        );
                    }
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let board = self.view_board();
        let status = if self.game.game_over {
            text(format!(
                "Game Over! Winner: {:?}",
                self.game.winner.unwrap()
            ))
        } else {
            text(format!("Current turn: {:?}", self.game.current_turn))
        };
        let content = column![board, status].spacing(20).padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

impl CheckersUI {
    fn view_board(&self) -> Element<'_, Message> {
        Canvas::new(BoardCanvas::new(
            &self.game,
            self.selected_piece,
            &self.available_moves,
        ))
        .width(Length::Fixed(480.0))
        .height(Length::Fixed(480.0))
        .into()
    }
}

impl Default for CheckersUI {
    fn default() -> Self {
        CheckersUI {
            game: GameManager::new(),
            selected_piece: None,
            available_moves: vec![],
        }
    }
}
