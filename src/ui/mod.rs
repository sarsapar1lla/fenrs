use eframe::egui;
use egui::{Key, Vec2};

mod board;
mod games;
mod ply;

use crate::model::{Game, PieceColour};

pub fn launch(games: Vec<Game>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(900.0, 700.0)),
        centered: true,
        ..Default::default()
    };
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let ply_painter = ply::Painter::new(6, 14.0);
        let game_painter = games::Painter::new(14.0);
        let current_game = &self.games[self.current_game];
        let current_board = &current_game.boards()[self.current_ply[self.current_game]];

        let bottom_panel_height = (frame.info().window_info.size.y * 0.66).min(200.0);

        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(bottom_panel_height)
            .show(ctx, |ui| {
                ui.horizontal_top(|ui| {
                    game_painter.list(ui, &self.games, self.current_game);
                    ui.separator();
                    ui.label("Some placeholder metadata");
                })
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ply_painter.list(
                    ui,
                    current_game.pgn().ply(),
                    self.current_ply[self.current_game],
                );
                ui.separator();
                ui.add(board::Painter::new(current_board, self.perspective));
            });

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

            // Quit
            if ctx.input(|i| i.key_pressed(Key::Q)) {
                frame.close();
            }
        });
    }
}
