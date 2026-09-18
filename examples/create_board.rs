use samolss_chess::board;

fn main() {
    let mut board =
        board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    board::print_board(&board);

    // We can also access the squares themselves with
    println!("{:?}", &board.squares);

    println!("Hurray!!!")
}
