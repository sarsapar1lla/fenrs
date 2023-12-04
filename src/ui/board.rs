use egui::{
    epaint::RectShape, Align2, Color32, FontId, Pos2, Rect, Rounding, Sense, Shape, Vec2, Widget,
};

use crate::model::{Board, Piece, PieceColour, Position, MAX_POSITION};

struct Dimensions {
    origin: Pos2,
    square_size: f32,
    font_size: f32,
}

pub struct Painter<'a> {
    board: &'a Board,
    perspective: PieceColour,
}

impl<'a> Painter<'a> {
    pub fn new(board: &'a Board, perspective: PieceColour) -> Self {
        Painter { board, perspective }
    }

    fn square(&self, painter: &egui::Painter, row: i8, col: i8, dimensions: &Dimensions) {
        let rect = Rect::from_min_max(
            self.square_min_position(row, col, dimensions),
            self.square_max_position(row, col, dimensions),
        );
        let fill_colour = if (row + col) % 2 == 0 {
            Color32::GRAY
        } else {
            Color32::DARK_GRAY
        };
        let rounding = Rounding::default();
        let rect_shape = RectShape::filled(rect, rounding, fill_colour);
        painter.add(Shape::Rect(rect_shape));
    }

    fn piece(
        &self,
        painter: &egui::Painter,
        row: i8,
        col: i8,
        piece: Piece,
        dimensions: &Dimensions,
    ) {
        let font_id = FontId::new(dimensions.font_size, egui::FontFamily::Proportional);
        painter.text(
            self.square_centre(row, col, dimensions),
            Align2::CENTER_CENTER,
            piece,
            font_id,
            Color32::BLACK,
        );
    }

    fn square_min_position(&self, row: i8, col: i8, dimensions: &Dimensions) -> Pos2 {
        let x_offset = dimensions.square_size * f32::from(col);
        let y_offset = dimensions.square_size * f32::from(row);
        Pos2::new(
            dimensions.origin.x + x_offset,
            dimensions.origin.y + y_offset,
        )
    }

    fn square_max_position(&self, row: i8, col: i8, dimensions: &Dimensions) -> Pos2 {
        let x_offset = dimensions.square_size * f32::from(col + 1);
        let y_offset = dimensions.square_size * f32::from(row + 1);
        Pos2::new(
            dimensions.origin.x + x_offset,
            dimensions.origin.y + y_offset,
        )
    }

    fn square_centre(&self, row: i8, col: i8, dimensions: &Dimensions) -> Pos2 {
        let x_offset = (dimensions.square_size * f32::from(col)) + (dimensions.square_size * 0.5);
        let y_offset = (dimensions.square_size * f32::from(row)) + (dimensions.square_size * 0.5);
        Pos2::new(
            dimensions.origin.x + x_offset,
            dimensions.origin.y + y_offset,
        )
    }
}

impl Widget for Painter<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let total_size = ui.available_width().min(ui.available_height());
        let dimensions = Dimensions {
            origin: ui.next_widget_position(),
            square_size: total_size / 8.0,
            font_size: total_size / 8.0,
        };
        let (_, response) = ui.allocate_at_least(Vec2::new(total_size, total_size), Sense::hover());
        let painter = ui.painter();
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
                self.square(painter, drawn_row, drawn_col, &dimensions);
                if let Some(&piece) = self.board.occupant(Position::new(row, col)) {
                    self.piece(painter, drawn_row, drawn_col, piece, &dimensions);
                }
            }
        }
        response
    }
}
