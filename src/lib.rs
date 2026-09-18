pub mod board;
pub mod move_piece;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perft_initial() {
        let mut board = board::create_board(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string(),
        );
        assert_eq!(move_piece::perft(&board, 1), 20);
        assert_eq!(move_piece::perft(&board, 2), 400);
        assert_eq!(move_piece::perft(&board, 3), 8902);
        assert_eq!(move_piece::perft(&board, 4), 197281);
    }

    #[test]
    fn perft_kiwipete() {
        let mut board = board::create_board(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 0".to_string(),
        );
        assert_eq!(move_piece::perft(&board, 1), 48);
        assert_eq!(move_piece::perft(&board, 2), 2039);
        assert_eq!(move_piece::perft(&board, 3), 97862);
    }

    #[test]
    fn perft_pos_3() {
        let mut board =
            board::create_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1".to_string());
        assert_eq!(move_piece::perft(&board, 1), 14);
        assert_eq!(move_piece::perft(&board, 2), 191);
        assert_eq!(move_piece::perft(&board, 3), 2812);
        assert_eq!(move_piece::perft(&board, 4), 43238);
    }

    #[test]
    fn perft_pos_4() {
        let mut board = board::create_board(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1".to_string(),
        );
        assert_eq!(move_piece::perft(&board, 1), 6);
        assert_eq!(move_piece::perft(&board, 2), 264);
        assert_eq!(move_piece::perft(&board, 3), 9467);
    }

    #[test]
    fn perft_pos_5() {
        let mut board = board::create_board(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8".to_string(),
        );
        assert_eq!(move_piece::perft(&board, 1), 44);
        assert_eq!(move_piece::perft(&board, 2), 1486);
        assert_eq!(move_piece::perft(&board, 3), 62379);
    }

    #[test]
    fn perft_pos_6() {
        let mut board = board::create_board(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10".to_string(),
        );
        assert_eq!(move_piece::perft(&board, 1), 46);
        assert_eq!(move_piece::perft(&board, 2), 2079);
        assert_eq!(move_piece::perft(&board, 3), 89890);
    }
}
