use egui::{Color32, FontId, Grid, RichText, ScrollArea};

use crate::model::{
    Check, MoveQualifier, Movement, Piece, PieceColour, PieceType, Ply, PlyMovement, Position,
    COLUMNS, ROWS,
};

pub struct Painter {
    columns: usize,
    font_size: f32,
    highlight_colour: Color32,
}

impl Painter {
    pub fn new(columns: usize, font_size: f32) -> Self {
        Painter {
            columns,
            font_size,
            highlight_colour: Color32::GOLD,
        }
    }

    pub fn list(&self, ui: &mut egui::Ui, ply_list: &[Ply], current_ply: usize) {
        let height = ui.available_height();
        let width = ui.available_width() / 2.0;
        ScrollArea::vertical()
            .min_scrolled_height(height)
            .max_height(height)
            .min_scrolled_width(width)
            .max_width(width)
            .show(ui, |ui| {
                Grid::new("ply_list_grid")
                    .num_columns(self.columns)
                    .striped(true)
                    .show(ui, |ui| {
                        for chunk in ply_list.chunks(self.columns) {
                            for ply in chunk {
                                let text_colour = if ply_list.iter().position(|p| p == ply).unwrap()
                                    == current_ply
                                {
                                    self.highlight_colour
                                } else {
                                    Color32::WHITE
                                };
                                ui.label(
                                    RichText::new(ply.to_string())
                                        .font(FontId::new(
                                            self.font_size,
                                            egui::FontFamily::Proportional,
                                        ))
                                        .color(text_colour),
                                );
                            }
                            ui.end_row();
                        }
                    });
            });
    }
}

impl ToString for Check {
    fn to_string(&self) -> String {
        match self {
            Check::Check => "+".to_string(),
            Check::Checkmate => "#".to_string(),
        }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match (self.colour(), self.piece_type()) {
            // Black pieces
            (PieceColour::Black, PieceType::Pawn) => "♟",
            (PieceColour::Black, PieceType::Knight) => "♞",
            (PieceColour::Black, PieceType::Bishop) => "♝",
            (PieceColour::Black, PieceType::Rook) => "♜",
            (PieceColour::Black, PieceType::Queen) => "♛",
            (PieceColour::Black, PieceType::King) => "♚",
            // White pieces
            (PieceColour::White, PieceType::Pawn) => "♙",
            (PieceColour::White, PieceType::Knight) => "♘",
            (PieceColour::White, PieceType::Bishop) => "♗",
            (PieceColour::White, PieceType::Rook) => "♖",
            (PieceColour::White, PieceType::Queen) => "♕",
            (PieceColour::White, PieceType::King) => "♔",
        }
        .to_string()
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        let row = ROWS.chars().nth(self.row() as usize).unwrap();
        let col = COLUMNS.chars().nth(self.col() as usize).unwrap();
        format!("{col}{row}")
    }
}

impl ToString for MoveQualifier {
    fn to_string(&self) -> String {
        match self {
            MoveQualifier::Col(col) => COLUMNS.chars().nth(*col as usize).unwrap().to_string(),
            MoveQualifier::Row(row) => ROWS.chars().nth(*row as usize).unwrap().to_string(),
            MoveQualifier::Position(position) => position.to_string(),
        }
    }
}

impl ToString for Ply {
    fn to_string(&self) -> String {
        match self.movement() {
            PlyMovement::KingsideCastle { colour, check } => {
                format_castle(self.move_number(), check.as_ref(), *colour, "O-O")
            }
            PlyMovement::QueensideCastle { colour, check } => {
                format_castle(self.move_number(), check.as_ref(), *colour, "O-O-O")
            }
            PlyMovement::Move {
                movement,
                qualifier,
                check,
                capture,
            } => format_move(
                self.move_number(),
                movement,
                qualifier.as_ref(),
                check.as_ref(),
                *capture,
                None,
            ),
            PlyMovement::Promotion {
                movement,
                promotes_to,
                qualifier,
                check,
                capture,
            } => format_move(
                self.move_number(),
                movement,
                qualifier.as_ref(),
                check.as_ref(),
                *capture,
                Some(promotes_to),
            ),
        }
    }
}

fn format_castle(
    move_number: i16,
    check: Option<&Check>,
    colour: PieceColour,
    castle_string: &str,
) -> String {
    let move_number = move_number_string(colour, move_number);
    let check_string = check.map_or(String::new(), ToString::to_string);
    format!("{move_number} {castle_string}{check_string}")
}

fn format_move(
    move_number: i16,
    movement: &Movement,
    qualifier: Option<&MoveQualifier>,
    check: Option<&Check>,
    capture: bool,
    promotes_to: Option<&PieceType>,
) -> String {
    let move_number = move_number_string(*movement.piece().colour(), move_number);
    let qualifier_string = qualifier.map_or(String::new(), ToString::to_string);
    let capture_string = if capture { "x" } else { "" };
    let check_string = check.map_or(String::new(), ToString::to_string);
    let promotion_string = match promotes_to {
        None => String::new(),
        Some(&piece_type) => Piece::new(*movement.piece().colour(), piece_type).to_string(),
    };
    format!(
        "{move_number} {}{qualifier_string}{capture_string}{}{promotion_string}{check_string}",
        movement.piece().to_string(),
        movement.position().to_string(),
    )
}

fn move_number_string(colour: PieceColour, move_number: i16) -> String {
    match colour {
        PieceColour::White => format!("{move_number}."),
        PieceColour::Black => String::new(),
    }
}
