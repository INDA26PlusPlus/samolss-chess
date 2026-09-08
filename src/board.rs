use std::fmt;

pub const WIDTH: usize = 8;
pub const HEIGHT: usize = 8;

// Enum describing what is on each square. Whether a piece has moved matters to some pieces: for
// example castling is not available if the king or the rook with which to castle has moved. The
// Pawns can also move 2 squares if they have yet to move.
#[derive(Clone, Copy)]
pub enum Piece {
    King { is_white: bool, has_moved: bool },
    Queen { is_white: bool },
    Rook { is_white: bool, has_moved: bool },
    Bishop { is_white: bool },
    Knight { is_white: bool },
    Pawn { is_white: bool, has_moved: bool },
    Empty,
}

impl Piece {
    pub fn is_white(&self) -> Option<bool> {
        match self {
            Piece::Empty => None,
            Piece::King { is_white, .. } => Some(*is_white),
            Piece::Queen { is_white, .. } => Some(*is_white),
            Piece::Rook { is_white, .. } => Some(*is_white),
            Piece::Bishop { is_white, .. } => Some(*is_white),
            Piece::Knight { is_white, .. } => Some(*is_white),
            Piece::Pawn { is_white, .. } => Some(*is_white),
        }
    }
    pub fn has_moved(&self) -> Option<bool> {
        match self {
            Piece::Empty => None,
            Piece::King { has_moved, .. } => Some(*has_moved),
            Piece::Queen { .. } => None,
            Piece::Rook { has_moved, .. } => Some(*has_moved),
            Piece::Bishop { .. } => None,
            Piece::Knight { .. } => None,
            Piece::Pawn { has_moved, .. } => Some(*has_moved),
        }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Piece::King { is_white: true, .. } => write!(f, "WK"),
            Piece::King {
                is_white: false, ..
            } => write!(f, "BK"),
            Piece::Queen { is_white: true } => write!(f, "WQ"),
            Piece::Queen { is_white: false } => write!(f, "BQ"),
            Piece::Rook { is_white: true, .. } => write!(f, "WR"),
            Piece::Rook {
                is_white: false, ..
            } => write!(f, "BR"),
            Piece::Bishop { is_white: true } => write!(f, "WB"),
            Piece::Bishop { is_white: false } => write!(f, "BB"),
            Piece::Knight { is_white: true } => write!(f, "WH"),
            Piece::Knight { is_white: false } => write!(f, "BH"),
            Piece::Pawn { is_white: true, .. } => write!(f, "WP"),
            Piece::Pawn {
                is_white: false, ..
            } => write!(f, "BP"),

            Piece::Empty => write!(f, "  "),
        }
    }
}
// The board is an Array of 8 arrays, each of those arrays containing 8 pieces.
// i    0   1   2   3   4   5   6   7
//      A   B   C   D   E   F   G   H
// 0 1 [R,  Kn, B,  Q,  K,  B,  Kn, R]  (White)
// 1 2 [P,  P,  P,  P,  P,  P,  P,  P]  (White)
// 2 3 [E,  E,  E,  E,  E,  E,  E,  E]
// 3 4 [E,  E,  E,  E,  E,  E,  E,  E]
// 4 5 [E,  E,  E,  E,  E,  E,  E,  E]
// 5 6 [E,  E,  E,  E,  E,  E,  E,  E]
// 6 7 [P,  P,  P,  P,  P,  P,  P,  P]  (Black)
// 7 8 [R,  Kn, B,  Q,  K,  B, Kn,  R]  (Black)
// So accessing the square H3 would be the indexes [7][2]
#[derive(Clone)]
pub struct Board {
    pub white_turn: bool,
    pub squares: [[Piece; WIDTH]; HEIGHT],
    pub history: Vec<(usize, usize)>,
}

pub fn create_board() -> Board {
    Board {
        white_turn: true,
        history: Vec::new(),
        squares: [
            [
                Piece::Rook {
                    is_white: true,
                    has_moved: false,
                },
                Piece::Knight { is_white: true },
                Piece::Bishop { is_white: true },
                Piece::Queen { is_white: true },
                Piece::King {
                    is_white: true,
                    has_moved: false,
                },
                Piece::Bishop { is_white: true },
                Piece::Knight { is_white: true },
                Piece::Rook {
                    is_white: true,
                    has_moved: false,
                },
            ],
            [
                Piece::Pawn {
                    is_white: true,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: true,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: true,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: true,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: true,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: true,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: true,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: true,
                    has_moved: false,
                },
            ],
            [
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
            ],
            [
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
            ],
            [
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
            ],
            [
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
                Piece::Empty,
            ],
            [
                Piece::Pawn {
                    is_white: false,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: false,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: false,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: false,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: false,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: false,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: false,
                    has_moved: false,
                },
                Piece::Pawn {
                    is_white: false,
                    has_moved: false,
                },
            ],
            [
                Piece::Rook {
                    is_white: false,
                    has_moved: false,
                },
                Piece::Knight { is_white: false },
                Piece::Bishop { is_white: false },
                Piece::Queen { is_white: false },
                Piece::King {
                    is_white: false,
                    has_moved: false,
                },
                Piece::Bishop { is_white: false },
                Piece::Knight { is_white: true },
                Piece::Rook {
                    is_white: false,
                    has_moved: false,
                },
            ],
        ],
    }
}

pub fn print_board(board: &Board) {
    println!("i A   B   C   D   E   F   G   H");
    for (i, row) in board.squares.iter().enumerate() {
        println!("{i}{:?}", row);
    }
}
