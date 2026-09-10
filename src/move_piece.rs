use crate::board;
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
    let mut is_castle = false;

    let piece = &board.squares[y][x];
    if Some(board.white_turn) != piece.is_white() {
        return Err("Turn is mismatched".to_string());
    }

    // This only checks if the move is allowed from the pieces own perspective, it does not include
    // if the king gets checked frome the move and will therefore have to be checked elsewhere
    let move_allowed = match piece {
        Piece::King { .. } => {
            let (move_allowed, is_castle_) = check_king_move(&board, old_square, new_square);
            is_castle = is_castle_;
            move_allowed
        }
        Piece::Queen { .. } => check_queen_move(&board, old_square, new_square),
        Piece::Rook { .. } => check_rook_move(&board, old_square, new_square),
        Piece::Bishop { .. } => check_bishop_move(&board, old_square, new_square),
        Piece::Knight { .. } => check_knight_move(&board, old_square, new_square),
        Piece::Pawn { .. } => check_pawn_move(&board, old_square, new_square),
        Piece::Empty => false,
    };

    if !move_allowed {
        return Err("Move not allowed".to_string());
    }

    let mut temp_board = board.clone();
    temp_board.white_turn = !&temp_board.white_turn;
    if is_castle {
        temp_board = castle(temp_board, old_square, new_square);
    } else {
        temp_board.squares[new_square.1][new_square.0] = temp_board.squares[y][x];
        temp_board.squares[y][x] = Piece::Empty;
        temp_board.history.push((old_square, new_square));
    }

    if king_is_checked(&temp_board, !temp_board.white_turn) {
        return Err("Move puts own king in check".to_string());
    }

    if is_castle {
        board = castle(board, old_square, new_square);
        return Ok(board);
    }
    board.white_turn = !&board.white_turn;
    board.squares[new_square.1][new_square.0] = board.squares[y][x];
    board.squares[y][x] = Piece::Empty;
    board.history.push((old_square, new_square));
    return Ok(board);
}

fn king_is_checked(board: &Board, king_is_white: bool) -> bool {
    for (y, row) in board.squares.iter().enumerate() {
        for (x, piece) in row.iter().enumerate() {
            if piece.is_white() != Some(king_is_white) {
                continue;
            }
            if !matches!(piece, Piece::King { .. }) {
                continue;
            }
            return square_is_checked(board, (x, y), king_is_white);
        }
    }
    panic!("King not found on board");
}

fn square_is_checked(board: &Board, square: (usize, usize), king_is_white: bool) -> bool {
    let squares = &board.squares;
    for (y, row) in squares.iter().enumerate() {
        for (x, piece) in row.iter().enumerate() {
            // println!("{:?}", piece);
            if piece.is_white() == Some(king_is_white) {
                continue;
            }
            let piece_checks_square = match piece {
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

            if !piece_checks_square {
                continue;
            }

            return true;
        }
    }
    return false;
}

fn king_threatens_square(king_square: (usize, usize), square: (usize, usize)) -> bool {
    king_square.0.abs_diff(square.0) <= 1 && king_square.1.abs_diff(square.1) <= 1
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

    return traverse_board(board, queen_square, square);
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

    return traverse_board(board, rook_square, square);
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

    return traverse_board(board, bishop_square, square);
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
    return x_diff == 1 && ((y_diff == -1 && is_white) || (y_diff == 1 && !is_white));
}

fn check_king_move(
    board: &Board,
    old_square: (usize, usize),
    new_square: (usize, usize),
) -> (bool, bool) {
    // Checks if the move of the king is allowed, returns (is_allowed, is_castle), the is_castle
    // part will only be non-false if the move itself is allowed
    let x_diff = old_square.0 as isize - new_square.0 as isize;
    let y_diff = old_square.1 as isize - new_square.1 as isize;

    let king = board.squares[old_square.1][old_square.0];
    let new_piece = board.squares[new_square.1][new_square.0];

    // This line results in an error, that does not affect compilation?????????????? Seems similar
    // to this https://github.com/rust-lang/rust-analyzer/issues/17441
    let new_piece_king_colored = new_piece.is_white() == king.is_white();
    let new_piece_moved = new_piece.has_moved() == Some(true);
    let is_castling = matches!(new_piece, Piece::Rook { .. })
        && king.has_moved() == Some(false)
        && new_piece_king_colored
        && !new_piece_moved;

    // King can only move one step unless castling
    if (old_square.0.abs_diff(new_square.0) > 1 || old_square.1.abs_diff(new_square.1) > 1)
        && !is_castling
    {
        return (false, false);
    }

    if is_castling {
        // When castling the king should only move 2 squares, unless some spinoff is being played
        let passed_square = if x_diff < 0 {
            (old_square.0 + 1, old_square.1)
        } else {
            (old_square.0 - 1, old_square.1)
        };
        let passed_piece = board.squares[passed_square.1][passed_square.0];
        let king_final_square = if x_diff < 0 {
            (passed_square.0 + 1, old_square.1)
        } else {
            (old_square.0 - 1, old_square.1)
        };
        let king_final_square_piece = board.squares[king_final_square.1][king_final_square.0];
        // You cannot pass/capture a piece when castling
        if !matches!(passed_piece, Piece::Empty) || !matches!(king_final_square_piece, Piece::Empty)
        {
            return (false, false);
        }

        if square_is_checked(board, passed_square, king.is_white() == Some(true)) {
            return (false, false);
        }

        // Long castle
        if x_diff.abs() == 4 {
            let rook_pass_square = if x_diff < 0 {
                (new_square.0 - 1, new_square.1)
            } else {
                (new_square.0 + 1, new_square.1)
            };
            let rook_pass_piece = board.squares[rook_pass_square.1][rook_pass_square.0];

            if !matches!(rook_pass_piece, Piece::Empty) {
                return (false, false);
            }
        }
        return (true, true);
    }

    let new_square_piece = &board.squares[new_square.0][new_square.1];

    if new_square_piece.is_white() == king.is_white() {
        return (false, false);
    }
    return (true, false);
}

fn check_queen_move(board: &Board, old_square: (usize, usize), new_square: (usize, usize)) -> bool {
    let x_diff = old_square.0 as isize - new_square.0 as isize;
    let y_diff = old_square.1 as isize - new_square.1 as isize;

    let slope = x_diff.abs() - y_diff.abs();

    // Isn't going straight nor is it going diagonally
    if !((x_diff == 0) ^ (y_diff == 0)) && slope != 0 {
        return false;
    }

    traverse_board(board, old_square, new_square)
}

fn check_rook_move(board: &Board, old_square: (usize, usize), new_square: (usize, usize)) -> bool {
    let x_diff = old_square.0 as isize - new_square.0 as isize;
    let y_diff = old_square.1 as isize - new_square.1 as isize;

    if !((x_diff == 0) ^ (y_diff == 0)) {
        return false;
    }

    traverse_board(board, old_square, new_square)
}

fn check_bishop_move(
    board: &Board,
    old_square: (usize, usize),
    new_square: (usize, usize),
) -> bool {
    let x_diff = old_square.0 as isize - new_square.0 as isize;
    let y_diff = old_square.1 as isize - new_square.1 as isize;

    if x_diff.abs() != y_diff.abs() {
        return false;
    }

    traverse_board(board, old_square, new_square)
}

fn check_knight_move(
    board: &Board,
    old_square: (usize, usize),
    new_square: (usize, usize),
) -> bool {
    let x_diff = old_square.0.abs_diff(new_square.0);
    let y_diff = old_square.1.abs_diff(new_square.1);

    let knight = board.squares[old_square.1][old_square.0];

    if !matches!(knight, Piece::Knight { .. }) {
        panic!("check_knight_move called for non-knight piece");
    }
    println!("{x_diff} {y_diff}");
    if !(x_diff == 2 && y_diff == 1) && !(x_diff == 1 && y_diff == 2) {
        return false;
    }

    let taken_piece = board.squares[new_square.1][new_square.0];

    if knight.is_white() == taken_piece.is_white() {
        return false;
    }

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

        if !matches!(passed_square, Piece::Empty) {
            return false;
        }

        if !matches!(board.squares[new_square.1][new_square.0], Piece::Empty) {
            return false;
        }
        return true;
    }
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

fn traverse_board(board: &Board, old_square: (usize, usize), new_square: (usize, usize)) -> bool {
    // Traverses the board from one square to another making sure there is nothing in between the
    let x_diff = old_square.0 as isize - new_square.0 as isize;
    let y_diff = old_square.1 as isize - new_square.1 as isize;

    let original_piece = board.squares[old_square.1][old_square.0];

    let mut temp_piece = &Piece::Empty;
    let mut temp_x = old_square.0;
    let mut temp_y = old_square.1;
    while matches!(temp_piece, &Piece::Empty) && temp_x < WIDTH && temp_y < HEIGHT {
        // temp_x -= x_diff.signum() as usize;
        if x_diff < 0 {
            temp_x += x_diff.signum().abs() as usize;
        } else {
            temp_x -= x_diff.signum().abs() as usize;
        }
        // temp_y -= y_diff.signum() as usize;
        if y_diff < 0 {
            temp_y += y_diff.signum().abs() as usize;
        } else {
            temp_y -= y_diff.signum().abs() as usize;
        }
        temp_piece = &board.squares[temp_y][temp_x];

        // Pieces are the same color
        if temp_piece.is_white() == original_piece.is_white() {
            return false;
        }

        if temp_x == new_square.0 && temp_y == new_square.1 {
            return true;
        }
    }
    panic!("FUUUUUUUUCK")
}

fn castle(mut board: Board, old_square: (usize, usize), new_square: (usize, usize)) -> Board {
    // Reminder to myself: I dont think we check whether there is anything between the rook and
    // the king, this should only matter for "O-O-O"
    let x_diff = old_square.0 as isize - new_square.0 as isize;
    // Should be 0 if castling

    let new_king_square = if x_diff.signum() == 1 {
        (old_square.0 - 2, old_square.1)
    } else {
        (old_square.0 + 2, old_square.1)
    };
    let new_rook_square = if x_diff.signum() == 1 {
        (new_king_square.0 + 1, new_king_square.1)
    } else {
        (new_king_square.0 - 1, new_king_square.1)
    };

    // Move king
    board.squares[new_king_square.1][new_king_square.0] = board.squares[old_square.1][old_square.0];
    board.squares[old_square.1][old_square.0] = Piece::Empty;

    //Move rook
    board.squares[new_rook_square.1][new_rook_square.0] = board.squares[new_square.1][new_square.0];
    board.squares[new_square.1][new_square.0] = Piece::Empty;

    board.history.push((old_square, new_square));

    return board;
}
