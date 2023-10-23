use eframe::egui;
use egui::*;

mod board;

use crate::model::{Game, PieceColour, Position, MAX_POSITION};

pub fn launch(games: Vec<Game>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native("FEN-rs", options, Box::new(|_cc| Box::new(App::new(games))))
}

struct App {
    games: Vec<Game>,
    current_game: usize,
    current_ply: Vec<usize>,
    perspective: PieceColour,
}

impl App {
    pub fn new(games: Vec<Game>) -> Self {
        let current_ply = (0..games.len()).map(|_| 0).collect();
        App {
            games,
            current_game: 0,
            current_ply,
            perspective: PieceColour::White,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Fenrs");

            let board_origin = Pos2::new(0.0, 0.0);
            let square_size = ui.available_height().min(ui.available_width()) / 8.0;     

            let board_painter = board::Painter::new(board_origin, square_size);
            let current_game = &self.games[self.current_game];
            let current_board = &current_game.boards()[self.current_ply[self.current_game]];

            for row in 0..8 {
                for col in 0..8 {
                    let drawn_row = match self.perspective {
                        PieceColour::White => MAX_POSITION - row,
                        PieceColour::Black => row,
                    };
                    let drawn_col = match self.perspective {
                        PieceColour::White => col,
                        PieceColour::Black => MAX_POSITION - col,
                    };
                    board_painter.square(ui.painter(), drawn_row, drawn_col);
                    if let Some(&piece) = current_board.occupant(Position::new(row as i8, col as i8)) {
                        board_painter.piece(ui.painter(), drawn_row, drawn_col, piece);
                    }
                }
            }

            // Cycle through games
            if ctx.input(|i| i.key_pressed(Key::W)) {
                if self.current_game > 0 {
                    self.current_game -= 1;
                }
            }
            if ctx.input(|i| i.key_pressed(Key::S)) {
                if self.current_game < self.games.len() - 1 {
                    self.current_game += 1;
                }
            }

            // Cycle through ply
            if ctx.input(|i| i.key_pressed(Key::A)) {
                if self.current_ply[self.current_game] > 0 {
                    let new_current_ply = self.current_ply[self.current_game] - 1;
                    self.current_ply[self.current_game] = new_current_ply;
                }
            }
            if ctx.input(|i| i.key_pressed(Key::D)) {
                let max_ply = self.games[self.current_game].total_ply() - 1;
                if self.current_ply[self.current_game] < max_ply {
                    let new_current_ply = self.current_ply[self.current_game] + 1;
                    self.current_ply[self.current_game] = new_current_ply;
                }
            }

            // Jump to beginning and end
            if ctx.input(|i| i.key_pressed(Key::Z)) {
                self.current_ply[self.current_game] = 0;
            }
            if ctx.input(|i| i.key_pressed(Key::X)) {
                let max_ply = self.games[self.current_game].total_ply() - 1;
                self.current_ply[self.current_game] = max_ply;
            }

            // Flip perspective
            if ctx.input(|i| i.key_pressed(Key::E)) {
                let new_perspective = match self.perspective {
                    PieceColour::White => PieceColour::Black,
                    PieceColour::Black => PieceColour::White,
                };
                self.perspective = new_perspective;
            }
        });
    }
}
