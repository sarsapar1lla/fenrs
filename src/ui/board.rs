use egui::{epaint::RectShape, Align2, Color32, FontId, Pos2, Rect, Rounding, Shape};

use crate::model::{Piece, PieceColour, MAX_POSITION, Position, Board};

pub struct Painter {
    origin: Pos2,
    size: f32,
    font_size: f32,
}

impl Painter {
    pub fn new(origin: Pos2, size: f32) -> Self {
        Painter {
            origin,
            size,
            font_size: size,
        }
    }

    pub fn board(&self, painter: &egui::Painter, board: &Board, perspective: PieceColour) {
        for row in 0..8 {
            for col in 0..8 {
                let drawn_row = match perspective {
                    PieceColour::White => MAX_POSITION - row,
                    PieceColour::Black => row,
                };
                let drawn_col = match perspective {
                    PieceColour::White => col,
                    PieceColour::Black => MAX_POSITION - col,
                };
                self.square(painter, drawn_row, drawn_col);
                if let Some(&piece) = board.occupant(Position::new(row, col)) {
                    self.piece(painter, drawn_row, drawn_col, piece);
                }
            }
        }
    }

    fn square(&self, painter: &egui::Painter, row: i8, col: i8) {
        let rect = Rect::from_min_max(
            self.square_min_position(row, col),
            self.square_max_position(row, col),
        );
        let fill_colour = if (row + col) % 2 == 0 {
            Color32::GRAY
        } else {
            Color32::BROWN
        };
        let rounding = Rounding::default();
        let rect_shape = RectShape::filled(rect, rounding, fill_colour);
        painter.add(Shape::Rect(rect_shape));
    }

    fn piece(&self, painter: &egui::Painter, row: i8, col: i8, piece: Piece) {
        let font_id = FontId::new(self.font_size, egui::FontFamily::Proportional);
        painter.text(
            self.square_centre(row, col),
            Align2::CENTER_CENTER,
            piece,
            font_id,
            Color32::BLACK,
        );
    }

    fn square_min_position(&self, row: i8, col: i8) -> Pos2 {
        let x_offset = self.size * f32::from(col);
        let y_offset = self.size * f32::from(row);
        Pos2::new(self.origin.x + x_offset, self.origin.y + y_offset)
    }

    fn square_max_position(&self, row: i8, col: i8) -> Pos2 {
        let x_offset = self.size * f32::from(col + 1);
        let y_offset = self.size * f32::from(row + 1);
        Pos2::new(self.origin.x + x_offset, self.origin.y + y_offset)
    }

    fn square_centre(&self, row: i8, col: i8) -> Pos2 {
        let x_offset = (self.size * f32::from(col)) + (self.size * 0.5);
        let y_offset = (self.size * f32::from(row)) + (self.size * 0.5);
        Pos2::new(self.origin.x + x_offset, self.origin.y + y_offset)
    }
}
