#![allow(dead_code)]

use fenrs::{execute_moves, launch, parse, Game};
use std::fs;

fn main() {
    let file = fs::read_to_string("./resources/test/acceptance/Candidates2022.pgn").unwrap();
    let pgns = parse(&file).unwrap();

    let games: Vec<Game> = pgns
        .into_iter()
        .map(|pgn| {
            let boards = execute_moves(pgn.fen().starting_board(), pgn.ply()).unwrap();
            Game::new(pgn, boards)
        })
        .collect();
    launch(games).unwrap();
}
