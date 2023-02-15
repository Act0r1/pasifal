#[derive(Clone, Copy, PartialEq)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

#[derive(Clone, Copy)]
struct Square {
    piece: Piece,
    color: Color,
}

#[derive(Clone, Copy)]
struct Board {
    squares: [[Square; 8]; 8],
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Black,
}

impl Board {
    fn new() -> Board {
        let mut board = Board {
            squares: [[Square {
                piece: Piece::Empty,
                color: Color::White,
            }; 8]; 8],
        };

        // Set up the starting position of the pieces
        board.squares[0][0] = Square {
            piece: Piece::Rook,
            color: Color::White,
        };
        board.squares[0][1] = Square {
            piece: Piece::Knight,
            color: Color::White,
        };
        board.squares[0][2] = Square {
            piece: Piece::Bishop,
            color: Color::White,
        };
        board.squares[0][3] = Square {
            piece: Piece::Queen,
            color: Color::White,
        };
        board.squares[0][4] = Square {
            piece: Piece::King,
            color: Color::White,
        };
        board.squares[0][5] = Square {
            piece: Piece::Bishop,
            color: Color::White,
        };
        board.squares[0][6] = Square {
            piece: Piece::Knight,
            color: Color::White,
        };
        board.squares[0][7] = Square {
            piece: Piece::Rook,
            color: Color::White,
        };

        for i in 0..8 {
            board.squares[1][i] = Square {
                piece: Piece::Pawn,
                color: Color::White,
            };
            board.squares[6][i] = Square {
                piece: Piece::Pawn,
                color: Color::Black,
            };
        }

        board.squares[7][0] = Square {
            piece: Piece::Rook,
            color: Color::Black,
        };
        board.squares[7][1] = Square {
            piece: Piece::Knight,
            color: Color::Black,
        };
        board.squares[7][2] = Square {
            piece: Piece::Bishop,
            color: Color::Black,
        };
        board.squares[7][3] = Square {
            piece: Piece::Queen,
            color: Color::Black,
        };
        board.squares[7][4] = Square {
            piece: Piece::King,
            color: Color::Black,
        };
        board.squares[7][5] = Square {
            piece: Piece::Bishop,
            color: Color::Black,
        };
        board.squares[7][6] = Square {
            piece: Piece::Knight,
            color: Color::Black,
        };
        board.squares[7][7] = Square {
            piece: Piece::Rook,
            color: Color::Black,
        };

        board
    }
}

