use crate::Game;
use egui::{Color32, FontId, Grid, RichText, ScrollArea};
use time::macros::format_description;
use time::Date;

pub struct Painter {
    font_size: f32,
    highlight_colour: Color32,
}

impl Painter {
    pub fn new(font_size: f32) -> Self {
        Painter {
            font_size,
            highlight_colour: Color32::GOLD,
        }
    }

    pub fn list(&self, ui: &mut egui::Ui, games: &[Game], current_game: usize) {
        let height = ui.available_height();
        let width = ui.available_width() / 2.0;
        ScrollArea::vertical()
            .min_scrolled_height(height)
            .max_height(height)
            .min_scrolled_width(width)
            .max_width(width)
            .show(ui, |ui| {
                Grid::new("game_list_grid")
                    .num_columns(1)
                    .striped(true)
                    .show(ui, |ui| {
                        for (idx, game) in games.iter().enumerate() {
                            let mut text = RichText::new(game.to_string())
                                .font(FontId::new(self.font_size, egui::FontFamily::Proportional));

                            if idx == current_game {
                                text = text
                                    .background_color(self.highlight_colour)
                                    .color(Color32::BLACK);
                            }
                            ui.label(text);
                            ui.end_row();
                        }
                    });
            });
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        let tags = self.pgn().tags();
        let white_player = tags.get_or_default("White", "Unknown");
        let black_player = tags.get_or_default("Black", "Unknown");
        let result = self.pgn().result().to_string();
        let location = tags.get("Site");
        let date = tags
            .get("Date")
            .map(|d| Date::parse(&d, &format_description!("[year].[month].[day]")));

        let mut string = String::new();
        string.push_str(&format!("{white_player} {result} {black_player}"));

        if let Some(location) = location {
            string.push_str(&format!(" - {location}"));
        };

        if let Some(Ok(date)) = date {
            let formatted = date.format(&format_description!("[year]-[month]-[day]"));
            if let Ok(formatted_date) = formatted {
                string.push_str(&format!(" - {formatted_date}"));
            }
        }

        string
    }
}
