use core::panic::PanicInfo;
use std::{fmt, string};

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
    pub fn set_moved(&mut self) {
        match self {
            Piece::Empty => {}
            Piece::King { has_moved, .. } => *has_moved = true,
            Piece::Queen { .. } => {}
            Piece::Rook { has_moved, .. } => *has_moved = true,
            Piece::Bishop { .. } => {}
            Piece::Knight { .. } => {}
            Piece::Pawn { has_moved, .. } => *has_moved = true,
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
    pub history: Vec<((usize, usize), (usize, usize))>,
    pub white_lost: bool,
    pub black_lost: bool,
    pub is_draw: bool,
}

pub fn create_board(fen_string: String) -> Board {
    let mut fen_parts = fen_string.split(" ");
    let (
        Some(positions),
        Some(turn),
        Some(castles),
        Some(en_passant_square),
        Some(half_move_clock),
        Some(full_move_clock),
    ) = (
        fen_parts.next(),
        fen_parts.next(),
        fen_parts.next(),
        fen_parts.next(),
        fen_parts.next(),
        fen_parts.next(),
    )
    else {
        panic!("Invalid fen string")
    };

    let white_turn = turn == "w";

    let castle_parts = castles.chars();
    let mut castle_sides = [false, false, false, false];
    for castle in castle_parts {
        match castle {
            'K' => castle_sides[0] = true,
            'Q' => castle_sides[1] = true,
            'k' => castle_sides[2] = true,
            'q' => castle_sides[3] = true,
            '-' => continue,
            _ => panic!("Bad castle"),
        }
    }

    let mut history: Vec<((usize, usize), (usize, usize))> = Vec::new();

    if en_passant_square != "-" {
        let mut en_passant_coords = en_passant_square.chars();
        let (Some(en_passant_y_char), Some(en_passant_x_char)) =
            (en_passant_coords.next(), en_passant_coords.next())
        else {
            panic!("Bad en passant fen String");
        };

        let en_passant_x = match en_passant_y_char {
            'a' => 0 as usize,
            'b' => 1 as usize,
            'c' => 2 as usize,
            'd' => 3 as usize,
            'e' => 4 as usize,
            'f' => 5 as usize,
            'g' => 6 as usize,
            'h' => 7 as usize,
            _ => panic!("Bad en passant square"),
        };
        let en_passant_y = en_passant_x_char
            .to_digit(10)
            .expect("Bad en passant square") as usize
            - 1;

        history.push((
            (
                en_passant_x,
                if white_turn {
                    en_passant_y + 1
                } else {
                    en_passant_y - 1
                },
            ),
            (
                en_passant_x,
                if white_turn {
                    en_passant_y - 1
                } else {
                    en_passant_y + 1
                },
            ),
        ));
    }

    let rows = positions.split("/");
    let mut squares: [[Piece; 8]; 8] = [
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
    ];
    for (y, row) in rows.enumerate() {
        let mut skip = 0;
        let mut x: usize = 0 as usize;
        for (x_, piece) in row.chars().enumerate() {
            if let Some(skips) = piece.to_digit(10) {
                x += skips as usize;
                continue;
            }

            let new_piece = match piece {
                'K' => Piece::King {
                    is_white: true,
                    has_moved: false,
                },
                'Q' => Piece::Queen { is_white: true },
                'R' => Piece::Rook {
                    is_white: true,
                    has_moved: !(y == 7
                        && ((x == 0 && castle_sides[1]) || (x == 7 && castle_sides[0]))),
                },
                'B' => Piece::Bishop { is_white: true },
                'N' => Piece::Knight { is_white: true },
                'P' => Piece::Pawn {
                    is_white: true,
                    has_moved: y != 6,
                },
                'k' => Piece::King {
                    is_white: false,
                    has_moved: false,
                },
                'q' => Piece::Queen { is_white: false },
                'r' => Piece::Rook {
                    is_white: false,
                    has_moved: !(y == 0
                        && ((x == 0 && castle_sides[3]) || (x == 7 && castle_sides[2]))),
                },
                'b' => Piece::Bishop { is_white: false },
                'n' => Piece::Knight { is_white: false },
                'p' => Piece::Pawn {
                    is_white: false,
                    has_moved: y != 1,
                },
                _ => panic!("Bad Pieces"),
            };
            squares[7 - y][x] = new_piece;
            x += 1;
        }
    }
    return Board {
        white_turn: white_turn,
        history: history,
        squares: squares,
        white_lost: false,
        black_lost: false,
        is_draw: false,
    };
}

pub fn print_board(board: &Board) {
    println!("i A   B   C   D   E   F   G   H");
    for (i, row) in board.squares.iter().enumerate() {
        println!("{i}{:?}", row);
    }
}
