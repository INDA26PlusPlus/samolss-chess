use samolss_chess::{
    board::{self, create_board, print_board},
    move_piece,
};

fn main() {
    let mut board = create_board();

    print_board(&board);
    board = move_piece::move_piece(board, (4, 1), (4, 3)).expect("E");
    print_board(&board);
}
