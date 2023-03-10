use std::collections::HashSet;

use crate::board::Board;

#[derive(Debug)]
struct Move {
    from: (usize, usize),
    to: (usize, usize),
    promotion: bool,
}

enum StatusGame {
    Playing,
    End,
}

struct Game {
    status: StatusGame,
    white_pieces: HashSet<(i32, String)>,
    black_pieces: HashSet<(i32, String)>,
    board: Board,
}

impl Game {
    fn new() -> Self {
        Self {
            status: StatusGame::Playing,
            white_pieces: HashSet::from([
                (2, "Rook".to_string()),
                (8, "Pawn".to_string()),
                (2, "Bishop".to_string()),
                (2, "Knight".to_string()),
                (1, "Queen".to_string()),
                (1, "King".to_string()),
            ]),
            black_pieces: HashSet::from([
                (2, "Rook".to_string()),
                (8, "Pawn".to_string()),
                (2, "Bishop".to_string()),
                (2, "Knight".to_string()),
                (1, "Queen".to_string()),
                (1, "King".to_string()),
            ]),
            board: Board::new()
        }
    }
}

enum Moves {
    Illegal,
    Legal,
}

pub fn make_move(board: &Board) {
    todo!()
}
