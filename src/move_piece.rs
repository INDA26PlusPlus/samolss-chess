use crate::board::Board;
use crate::board::Piece;
use crate::board::create_board;

pub fn move_piece(
    board: &Board,
    old_square: (usize, usize),
    new_square: (usize, usize),
) -> Result<Board, String> {
    let x = old_square.0;
    let y = old_square.1;
    println!("{:?}", board.squares[y][x]);

    let piece = &board.squares[y][x];
    println!("piece: {:?}", piece);
    if matches!(piece, Piece::Empty) {
        return Err("Old square is empty".to_string());
    }

    if Some(board.white_turn) != piece.is_white() {
        return Err("Turn is mismatched".to_string());
    }

    let new_board = create_board();
    return Ok(new_board);
}

fn move_pawn(board: &Board, old_square: (usize, usize), new_square: (usize, usize)) {
    println!("Move Pawn");
}
