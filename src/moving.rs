use crate::board::{Board};

#[derive(Debug)]
struct Move {
    from: (usize, usize),
    to: (usize, usize),
    promotion: bool
}

enum Moves {
  Illegal,
  Legal
}

pub fn make_move(board:&Board) {
  todo!()
}
