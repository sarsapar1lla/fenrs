use eframe::egui;
use egui::{Key, Pos2};

mod board;
mod ply_list;

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
            let board_origin = Pos2::new(40.0, 40.0);
            let square_size = (ui.available_height() - 40.0).min(ui.available_width() - 40.0) / 8.0;

            // let ply_painter = ply_list::Painter::new(ply_origin, )
            let board_painter = board::Painter::new(board_origin, square_size);
            let current_game = &self.games[self.current_game];
            let current_board = &current_game.boards()[self.current_ply[self.current_game]];

            let current_ply_text = if self.current_ply[self.current_game] > 0 {
                current_game.pgn().ply()[self.current_ply[self.current_game] - 1].to_string()
            } else {
                String::new()
            };
            ui.label(format!("Current ply: {current_ply_text}"));

            board_painter.board(ui.painter(), current_board, self.perspective);

            // Cycle through games
            if ctx.input(|i| i.key_pressed(Key::W)) && self.current_game > 0 {
                self.current_game -= 1;
            }
            if ctx.input(|i| i.key_pressed(Key::S)) && self.current_game < self.games.len() - 1 {
                self.current_game += 1;
            }

            // Cycle through ply
            if ctx.input(|i| i.key_pressed(Key::A)) && self.current_ply[self.current_game] > 0 {
                let new_current_ply = self.current_ply[self.current_game] - 1;
                self.current_ply[self.current_game] = new_current_ply;
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
