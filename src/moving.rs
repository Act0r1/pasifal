use std::collections::HashSet;

use crate::board::Board;

#[derive(Debug)]
struct Move {
    from: (usize, usize),
    to: (usize, usize),
    promotion: Option<String>,
}

enum StatusGame {
    Playing,
    End,
}

enum KingStatus {
  Check,
  Mate,
  NotCheck,
}

struct Game {
    status: StatusGame,
    white_pieces: HashSet<(i32, String)>,
    black_pieces: HashSet<(i32, String)>,
    board: Board,
    king:KingStatus
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
            board: Board::new(),
            king:KingStatus::NotCheck,
        }
    }
    fn is_end(&self) -> bool {
        match &self.status {
            StatusGame::End => true,
            _ => false
        }
    }
}


pub fn make_move(board: &Board) {
    todo!()
}
