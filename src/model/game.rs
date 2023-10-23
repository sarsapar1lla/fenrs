use super::{Board, Pgn};

pub struct Game {
    pgn: Pgn,
    boards: Vec<Board>,
}

impl Game {
    pub fn new(pgn: Pgn, boards: Vec<Board>) -> Self {
        Game { pgn, boards }
    }

    pub fn pgn(&self) -> &Pgn {
        &self.pgn
    }

    pub fn boards(&self) -> &[Board] {
        &self.boards
    }

    pub fn total_ply(&self) -> usize {
        self.boards.len()
    }
}
