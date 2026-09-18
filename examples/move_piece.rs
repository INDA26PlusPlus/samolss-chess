use samolss_chess::board;
use samolss_chess::move_piece;

fn main() {
    let mut board =
        board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    // The e is the 5th letter, hence the index will be 5 - 1 = 4 for the x coordinate
    // 2 will similarily give us 1 as 2 - 1 = 1
    let e2_square = (4, 1);
    let e4_square = (4, 3);

    // To move a piece, we need a board, the square the current piece is on, the square we want to
    // move to, and what we want to promote our piece to (will only be used when a pawn is actually
    // being promoted, but it needs to be supplied for now).
    board = move_piece::move_piece(board, e2_square, e4_square, 'q').expect("Move was invalid");

    board::print_board(&board);

    println!("Hurray!!");
}
