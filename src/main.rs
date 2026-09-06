use samolss_chess::{
    board::{self, create_board, print_board},
    move_piece,
};

fn main() {
    let board = create_board();

    print_board(&board);
    let new_board = move_piece::move_piece(&board, (4, 6), (4, 3)).expect("E");
}
