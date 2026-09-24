# samolss-chess

Chess library written in rust

## Game represemtation

### The Piece

The piece is an Enum defined as such:

```rust
pub enum Piece {
    King { is_white: bool, has_moved: bool },
    Queen { is_white: bool },
    Rook { is_white: bool, has_moved: bool },
    Bishop { is_white: bool },
    Knight { is_white: bool },
    Pawn { is_white: bool, has_moved: bool },
    Empty,
}
```

Each option is a Piece type (including empty squares). All of the piece types keep track of their color, if its relevant for the piece (as it is for kings, rooks and pawns) they also store whether the piece has moved.

### The Board

The board is a struct defined as such:

```rust
pub struct Board {
    pub white_turn: bool,
    pub squares: [[Piece; WIDTH]; HEIGHT],
    pub history: Vec<((usize, usize), (usize, usize))>,
    pub white_lost: bool,
    pub black_lost: bool,
    pub is_draw: bool,
}
```

This stores basically all the gamestate.
white_turn is a bool, if true it is currently whites turn, otherwise it is blacks turn
squares is an 8x8 array containing the [Piece enum](#the-piece)
history is a vector containing all the moves that have been played. Each move is in the form of a tuple with three values. The first value is the old square in the form of a tuple with (x, y). The second value is the new square in the form of a tuple with (x, y). And the third value is a char, representing the promotion piece: valid options inlcude 'q' (queen), 'r' (rook), 'b' (bishop), 'n' (knight). The promotion piece only matters if a pawn has advanced all the way to the opposite end of the board. Note that history may not be complete if a fen string that is not the start position is used to create the board.
The remaining fields describe the end state of the game and are updated when move_piece::move_piece is called.

## Public functions

### Board

```rust
create_board(fen_string: String) -> Board
```

This function returns a [Board](#the-board). It takes a parameter fen_string, and will be parsed (quite badly), all the fields parts of the fen string need to be a value. See [wikipedia on fen strings](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)

```rust
print_board(board: &Board)
```

Prints out the current state of the board.
Example output for start position:

```
i A   B   C   D   E   F   G   H
0[WR, WH, WB, WQ, WK, WB, WH, WR]
1[WP, WP, WP, WP, WP, WP, WP, WP]
2[  ,   ,   ,   ,   ,   ,   ,   ]
3[  ,   ,   ,   ,   ,   ,   ,   ]
4[  ,   ,   ,   ,   ,   ,   ,   ]
5[  ,   ,   ,   ,   ,   ,   ,   ]
6[BP, BP, BP, BP, BP, BP, BP, BP]
7[BR, BH, BB, BQ, BK, BB, BH, BR]
```

### move_piece

```rust
move_piece(
    mut board: Board,
    old_square: (usize, usize),
    new_square: (usize, usize),
    promotion_piece: char,
) -> Result<Board, String>
```

This function moves a piece from one square to another if and only if the move is completely legal. It returns a result of the new board or an error message. As parameters: it takes the board on which to move the piece, the old and the new squares, each in the form of a tuple with (x, y), The promotion piece is a char describing what a pawn should ('q' for queen, 'r' for rook, 'b' for bishop and 'n' for knigh).

```rust
gen_all_moves(board: &Board) -> Vec<((usize, usize), (usize, usize), char)>
```

Generates all the possible moves for all pieces (only the piece of the turn color). It returns a vector of tuples, each tuple has (old_square: (usize, usize), new_square: (usize, usize), promotion_piece: char). It takes a parameter board: Board.

```rust
gen_moves_for_piece(
    board: &Board,
    square: (usize, usize),
) -> Vec<((usize, usize), (usize, usize), char)>
```

Generates all possible moves for whatever piece is present at a specific square. It returns a vector of tuples, each tuple has (old_square: (usize, usize), new_square: (usize, usize), promotion_piece: char). It takes two parameters: the board and the square of the piece.

```rust
perft(board: &Board, depth: i32) -> i32
```

Perft function for testing, please refer to [chess programming wiki](https://chessprogramming.org/Perft)

## Examples

Examples are available in [examples directory](./examples/)
