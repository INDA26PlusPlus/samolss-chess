use samolss_chess::{
    board::{self, create_board, print_board},
    move_piece::{self, move_piece},
};

use std::{char, io};

fn main() {
    let mut board = create_board();
    print_board(&board);
    while (true) {
        let (old, new) = handle_input();
        board = move_piece(board, old, new).expect("Fuuck");
        print_board(&board);
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
        'a' => 7 as usize,
        'b' => 6 as usize,
        'c' => 5 as usize,
        'd' => 4 as usize,
        'e' => 3 as usize,
        'f' => 2 as usize,
        'g' => 1 as usize,
        'h' => 0 as usize,
        _ => panic!("Bad move"),
    };
    println!("{} {}", old_square_x, old_square_y);
    let new_square_y = new_square[1].to_digit(10).expect(("F")) as usize - 1;
    let new_square_x = match new_square[0] {
        'a' => 7 as usize,
        'b' => 6 as usize,
        'c' => 5 as usize,
        'd' => 4 as usize,
        'e' => 3 as usize,
        'f' => 2 as usize,
        'g' => 1 as usize,
        'h' => 0 as usize,
        _ => panic!("Bad move"),
    };

    return ((old_square_x, old_square_y), (new_square_x, new_square_y));
}
