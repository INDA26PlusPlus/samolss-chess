use samolss_chess::{
    board::{self, create_board, print_board},
    move_piece,
};

fn main() {
    let mut board = create_board();

    print_board(&board);
    board = move_piece::move_piece(board, (4, 1), (4, 3)).expect("E");
    print_board(&board);
    board = move_piece::move_piece(board, (5, 6), (5, 4)).expect("E");
    print_board(&board);
    board = move_piece::move_piece(board, (4, 3), (5, 4)).expect("E");
    print_board(&board);
    board = move_piece::move_piece(board, (1, 6), (1, 4)).expect("E");
    print_board(&board);
    board = move_piece::move_piece(board, (7, 6), (7, 4)).expect("E");
    print_board(&board);
}
