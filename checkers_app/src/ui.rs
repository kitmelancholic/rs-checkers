use iced::{
    Color, Element, Length,
    widget::{column, container, row, text},
};

pub fn main() -> iced::Result {
    iced::run("Checkers - rs-checkers", update, view)
}

fn update(state: &mut CheckersUI, message: Message) {
    state.update(message);
}

fn view(state: &CheckersUI) -> Element<'_, Message> {
    state.view()
}

#[derive(Default, Debug, Clone)]
pub struct CheckersUI {
    selected_piece: Option<(usize, usize)>, // Track the selected piece (row, col)
    available_moves: Vec<(usize, usize)>,   // Highlight available moves
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    PieceSelected(usize, usize),
    PieceMoved(usize, usize),
}

impl CheckersUI {
    fn update(&mut self, message: Message) {
        match message {
            Message::PieceSelected(row, col) => {
                self.selected_piece = Some((row, col));
                self.available_moves = self.calculate_available_moves(row, col);
            }
            Message::PieceMoved(_row, _col) => {
                // Handle piece movement logic here
                self.selected_piece = None;
                self.available_moves.clear();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let board = self.view_board();
        let content = column![board].spacing(20).padding(20);

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
        let mut rows = column![];

        for row in 0..8 {
            let mut cells = row![];

            for col in 0..8 {
                let is_selected = self.selected_piece == Some((row, col));
                let is_available = self.available_moves.contains(&(row, col));
                let cell_color = if (row + col) % 2 == 0 {
                    Color::from_rgb(0.93, 0.93, 0.82) // Light cell (beige)
                } else {
                    Color::from_rgb(0.46, 0.59, 0.34) // Dark cell (green)
                };

                let mut cell_container = container(text(""))
                    .width(Length::Fixed(60.0))
                    .height(Length::Fixed(60.0))
                    .style(move |_theme| {
                        container::Style {
                            background: Some(cell_color.into()),
                            border: iced::Border {
                                color: if is_selected {
                                    Color::from_rgb(1.0, 0.84, 0.0) // Gold border for selected
                                } else {
                                    Color::from_rgb(0.2, 0.2, 0.2)
                                },
                                width: if is_selected { 3.0 } else { 1.0 },
                                radius: 0.0.into(),
                            },
                            ..Default::default()
                        }
                    });

                // Add a visual indicator for available moves
                if is_available {
                    cell_container = container(
                        container(text(""))
                            .width(Length::Fixed(20.0))
                            .height(Length::Fixed(20.0))
                            .style(|_theme| container::Style {
                                background: Some(Color::from_rgba(0.0, 0.5, 1.0, 0.6).into()),
                                border: iced::Border {
                                    radius: 10.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }),
                    )
                    .width(Length::Fixed(60.0))
                    .height(Length::Fixed(60.0))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .style(move |_theme| container::Style {
                        background: Some(cell_color.into()),
                        border: iced::Border {
                            color: Color::from_rgb(0.2, 0.2, 0.2),
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    });
                }

                cells = cells.push(cell_container);
            }

            rows = rows.push(cells);
        }

        rows.into()
    }

    fn calculate_available_moves(&self, _row: usize, _col: usize) -> Vec<(usize, usize)> {
        vec![(2, 3), (3, 4), (4, 5)]
    }
}
