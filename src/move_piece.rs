use crate::board::Board;
use crate::board::HEIGHT;
use crate::board::Piece;
use crate::board::WIDTH;

pub fn move_piece(
    mut board: Board,
    old_square: (usize, usize),
    new_square: (usize, usize),
) -> Result<Board, String> {
    println!("Move_piece");
    let x = old_square.0;
    let y = old_square.1;

    let piece = &board.squares[y][x];
    println!("{:?}:, x:{x}, y:{y}", piece);
    if Some(board.white_turn) != piece.is_white() {
        return Err("Turn is mismatched".to_string());
    }

    let move_allowed = match piece {
        Piece::King { .. } => check_king_move(&board, old_square, new_square),
        Piece::Queen { .. } => check_queen_move(&board, old_square, new_square),
        Piece::Rook { .. } => check_rook_move(&board, old_square, new_square),
        Piece::Bishop { .. } => check_bishop_move(&board, old_square, new_square),
        Piece::Knight { .. } => check_knight_move(&board, old_square, new_square),
        Piece::Pawn { .. } => check_pawn_move(&board, old_square, new_square),
        Piece::Empty => false,
    };

    if move_allowed {
        board.white_turn = !&board.white_turn;
        board.squares[new_square.1][new_square.0] = board.squares[y][x];
        board.squares[y][x] = Piece::Empty;
        return Ok(board);
    }

    Err("Unallowed move".to_string())
}

fn square_is_checked(board: &Board, square: (usize, usize), king_is_white: bool) {
    let squares = &board.squares;
    for (y, row) in squares.iter().enumerate() {
        for (x, piece) in row.iter().enumerate() {
            if piece.is_white() == Some(king_is_white) {
                continue;
            }
            match piece {
                Piece::Empty => continue,
                Piece::King { .. } => king_threatens_square((x, y), square),
                Piece::Queen { .. } => queen_threatens_square(board, (x, y), square),
                Piece::Rook { .. } => rook_threatens_square(board, (x, y), square),
                Piece::Bishop { .. } => bishop_threatens_square(board, (x, y), square),
                Piece::Knight { .. } => knight_threatens_square((x, y), square),
                Piece::Pawn { .. } => {
                    pawn_threatens_square((x, y), square, piece.is_white() == Some(true))
                }
            };
        }
    }
}

fn king_threatens_square(king_square: (usize, usize), square: (usize, usize)) -> bool {
    king_square.0.abs_diff(square.0) < 1 && king_square.1.abs_diff(square.1) < 1
}

fn queen_threatens_square(
    board: &Board,
    queen_square: (usize, usize),
    square: (usize, usize),
) -> bool {
    // Queen can move either in straight lines (x or y is the same), or diagonals (add/subtract the same amount to both x
    // and y)
    //
    // If x diff or y diff are 0, then the queen will move in a straight line
    let x_diff = queen_square.0 as isize - square.0 as isize;
    let y_diff = queen_square.1 as isize - square.1 as isize;
    // If the slope (not really slope) is 0 the queen will go equal lengths up/down and will hence
    // have moved diagonally.
    let slope = x_diff.abs() - y_diff.abs();

    if x_diff != 0 && y_diff != 0 && slope != 0 {
        return false;
    }

    // Step towards the square one at a time until we reach a non-empty square or the square
    let mut temp_piece = &Piece::Empty;
    let mut temp_x = queen_square.0;
    let mut temp_y = queen_square.1;
    while matches!(temp_piece, &Piece::Empty)
        && temp_x > 0
        && temp_x < WIDTH
        && temp_y > 0
        && temp_y < HEIGHT
    {
        temp_x += x_diff.signum() as usize;
        temp_y += y_diff.signum() as usize;
        temp_piece = &board.squares[temp_y][temp_x];

        if temp_x == square.0 && temp_y == square.1 {
            return true;
        }

        if !matches!(temp_piece, &Piece::Empty) {
            return false;
        }
    }
    // Something went wrong
    panic!("Fuuuuuuuck");
}

fn rook_threatens_square(
    board: &Board,
    rook_square: (usize, usize),
    square: (usize, usize),
) -> bool {
    let x_diff = rook_square.0 as isize - square.0 as isize;
    let y_diff = rook_square.1 as isize - square.1 as isize;

    if x_diff != 0 && y_diff != 0 {
        return false;
    }

    // Should probably break this out into a separate function, since it handles straights and
    // diagonals
    let mut temp_piece = &Piece::Empty;
    let mut temp_x = rook_square.0;
    let mut temp_y = rook_square.1;
    while matches!(temp_piece, &Piece::Empty)
        && temp_x > 0
        && temp_x < WIDTH
        && temp_y > 0
        && temp_y < HEIGHT
    {
        temp_x += x_diff.signum() as usize;
        temp_y += y_diff.signum() as usize;
        temp_piece = &board.squares[temp_y][temp_x];

        if temp_x == square.0 && temp_y == square.1 {
            return true;
        }

        if !matches!(temp_piece, &Piece::Empty) {
            return false;
        }
    }
    // Something went wrong
    panic!("Fuuuuuuuck");
}

fn bishop_threatens_square(
    board: &Board,
    bishop_square: (usize, usize),
    square: (usize, usize),
) -> bool {
    let x_diff = bishop_square.0 as isize - square.0 as isize;
    let y_diff = bishop_square.1 as isize - square.1 as isize;
    let slope = x_diff.abs() - y_diff.abs();

    if slope != 0 {
        return false;
    }

    let mut temp_piece = &Piece::Empty;
    let mut temp_x = bishop_square.0;
    let mut temp_y = bishop_square.1;
    while matches!(temp_piece, &Piece::Empty)
        && temp_x > 0
        && temp_x < WIDTH
        && temp_y > 0
        && temp_y < HEIGHT
    {
        temp_x += x_diff.signum() as usize;
        temp_y += y_diff.signum() as usize;
        temp_piece = &board.squares[temp_y][temp_x];

        if temp_x == square.0 && temp_y == square.1 {
            return true;
        }

        if !matches!(temp_piece, &Piece::Empty) {
            return false;
        }
    }
    // Something went wrong
    panic!("Fuuuuuuuck");
}

fn knight_threatens_square(knight_square: (usize, usize), square: (usize, usize)) -> bool {
    let x_diff = knight_square.0.abs_diff(square.0);
    let y_diff = knight_square.1.abs_diff(square.1);

    if (x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2) {
        return true;
    }
    return false;
}

fn pawn_threatens_square(
    pawn_square: (usize, usize),
    square: (usize, usize),
    is_white: bool,
) -> bool {
    let x_diff = pawn_square.0.abs_diff(square.0);
    let y_diff = pawn_square.1 as isize - square.1 as isize;

    // if y_diff < 0 and is_white were going forward
    return x_diff == 1 && ((y_diff == -1 && is_white) || (y_diff != 1 && !is_white));
}

fn check_king_move(board: &Board, old_square: (usize, usize), new_square: (usize, usize)) -> bool {
    // King can only move one step
    if old_square.0.abs_diff(new_square.0) > 1 || old_square.1.abs_diff(new_square.1) > 1 {
        return false;
    }

    let new_square_piece = &board.squares[new_square.0][new_square.1];
    let new_square_occupied = !(matches!(new_square_piece, Piece::Empty));

    return true;
}

fn check_queen_move(board: &Board, old_square: (usize, usize), new_square: (usize, usize)) -> bool {
    return true;
}

fn check_rook_move(board: &Board, old_square: (usize, usize), new_square: (usize, usize)) -> bool {
    return true;
}

fn check_bishop_move(
    board: &Board,
    old_square: (usize, usize),
    new_square: (usize, usize),
) -> bool {
    return true;
}

fn check_knight_move(
    board: &Board,
    old_square: (usize, usize),
    new_square: (usize, usize),
) -> bool {
    return true;
}

fn check_pawn_move(board: &Board, old_square: (usize, usize), new_square: (usize, usize)) -> bool {
    let x_diff = old_square.0.abs_diff(new_square.0);
    let y_diff = old_square.1 as isize - new_square.1 as isize;

    let pawn = board.squares[old_square.1][old_square.0];

    if !matches!(pawn, Piece::Pawn { .. }) {
        panic!("check pawn move called on non-pawn piece")
    }

    // Pawn is going the wrong way
    if (y_diff < 0 && pawn.is_white() != Some(true))
        || (y_diff > 0 && pawn.is_white() == Some(true))
    {
        return false;
    }

    if y_diff.abs() > 2 {
        return false;
    }
    println!("2_step");
    if y_diff.abs() == 2 {
        if pawn.has_moved() == Some(true) {
            return false;
        }
        if x_diff != 0 {
            return false;
        }
        let passed_y = (old_square.1 as isize - y_diff / 2) as usize;
        let passed_x = old_square.0;
        let passed_square = board.squares[passed_y][passed_x];

        println!("{:?}, {:?}, {:?}", y_diff, old_square, passed_square);

        if !matches!(passed_square, Piece::Empty) {
            return false;
        }

        if !matches!(board.squares[new_square.1][new_square.0], Piece::Empty) {
            return false;
        }
        return true;
    }
    println!("1_step");
    if y_diff.abs() == 1 {
        if x_diff == 1 {
            let piece_to_take = board.squares[new_square.1][new_square.0];
            // You cannot take an empty piece (this will change a bit with en passasnt) or the king
            if matches!(piece_to_take, Piece::Empty) || matches!(piece_to_take, Piece::King { .. })
            {
                return false;
            }

            if pawn.is_white() == piece_to_take.is_white() {
                return false;
            }

            return true;
        }

        if !matches!(board.squares[new_square.1][new_square.0], Piece::Empty) {
            return false;
        }

        return true;
    }

    return false;
}
