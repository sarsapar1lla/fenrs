use egui::{Shape, epaint::RectShape, Rect, Pos2, Color32, Rounding, Align2, FontId};

use crate::model::Piece;

pub struct Painter {
    origin: Pos2,
    size: f32,
    font_size: f32
}

impl Painter {
    pub fn new(origin: Pos2, size: f32) -> Self {
        Painter { origin, size, font_size: size }
    }

    pub fn square(&self, painter: &egui::Painter, row: i8, col: i8) {
        let rect = Rect::from_min_max(
            self.square_min_position(row, col),
            self.square_max_position(row, col)
        );
        let fill_colour = if (row + col) % 2 == 0 { Color32::GRAY } else { Color32::BROWN };
        let rounding = Rounding::default();
        let rect_shape = RectShape::filled(rect, rounding, fill_colour);
        painter.add(Shape::Rect(rect_shape));
    }

    pub fn piece(&self, painter: &egui::Painter, row: i8, col: i8, piece: Piece) {
        let font_id = FontId::new(self.font_size, egui::FontFamily::Proportional);
        painter.text(self.square_centre(row, col), Align2::CENTER_CENTER, piece.unicode(), font_id, Color32::BLACK);
    }

    fn square_min_position(&self, row: i8, col: i8) -> Pos2 {
        let x_offset = self.size * col as f32;
        let y_offset = self.size * row as f32;
        Pos2::new(self.origin.x + x_offset, self.origin.y + y_offset)
    }

    fn square_max_position(&self, row: i8, col: i8) -> Pos2 {
        let x_offset = self.size * (col + 1) as f32;
        let y_offset = self.size * (row + 1) as f32;
        Pos2::new(self.origin.x + x_offset, self.origin.y + y_offset)
    }

    fn square_centre(&self, row: i8, col: i8) -> Pos2 {
        let x_offset = (self.size * col as f32) + (self.size * 0.5);
        let y_offset = (self.size * row as f32) + (self.size * 0.5);
        Pos2::new(self.origin.x + x_offset, self.origin.y + y_offset)
    }
}
