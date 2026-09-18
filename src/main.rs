use samolss_chess::{
    board::{self, create_board, print_board},
    move_piece::{self, gen_all_moves, move_piece, perft},
};

use std::{char, io};

fn main() {
    let mut board = create_board(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string(),
    );
    // let mut board =
    //     create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    print_board(&board);
    println!("{:?}", perft(&board, 5));
    while (true) {
        let (old, new) = handle_input();
        board = move_piece(board, old, new, 'q').expect("Fuuck");
        print_board(&board);
        println!("Is white: {}", board.white_turn);
    }
}

fn handle_input() -> ((usize, usize), (usize, usize)) {
    // Taken from https://doc.rust-lang.org/std/io/struct.Stdin.html
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer).expect("Bad input");
    println!("{}", buffer);
    let chars: Vec<char> = buffer.chars().collect();
    let (old_square, new_square) = chars.split_at(2);

    println!("{}", old_square[0]);
    let old_square_y = old_square[1].to_digit(10).expect("EEE") as usize - 1;
    let old_square_x = match old_square[0] {
        'a' => 0 as usize,
        'b' => 1 as usize,
        'c' => 2 as usize,
        'd' => 3 as usize,
        'e' => 4 as usize,
        'f' => 5 as usize,
        'g' => 6 as usize,
        'h' => 7 as usize,
        _ => panic!("Bad move"),
    };
    println!("{} {}", old_square_x, old_square_y);
    let new_square_y = new_square[1].to_digit(10).expect(("F")) as usize - 1;
    let new_square_x = match new_square[0] {
        'a' => 0 as usize,
        'b' => 1 as usize,
        'c' => 2 as usize,
        'd' => 3 as usize,
        'e' => 4 as usize,
        'f' => 5 as usize,
        'g' => 6 as usize,
        'h' => 7 as usize,
        _ => panic!("Bad move"),
    };

    return ((old_square_x, old_square_y), (new_square_x, new_square_y));
}
