use samolss_chess::board;
use samolss_chess::move_piece;
fn main() {
    let mut board =
        board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    // This will generate all LEGAL moves, each move consisting of the old square, the new square
    // and a char denoting what the promotion piece should be.
    println!("{:?}", move_piece::gen_all_moves(&board));

    println!("");

    // This will generate all LEGAL moves for a specific piece on the board (denoted by the square
    // its on). In this case the piece is the pawn on e2
    println!("{:?}", move_piece::gen_moves_for_piece(&board, (4, 1)));

    // Good to know is that when castling the old square should be the kings square and the new
    // square should be the rooks square.
}
