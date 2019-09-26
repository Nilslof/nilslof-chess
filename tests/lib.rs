extern crate nilslofchess;

#[cfg(test)]
mod game {
    use nilslofchess::Game;
    use nilslofchess::Colour;
    use nilslofchess::Piece;
    use nilslofchess::PieceType;
    use nilslofchess::MoveType;

    fn draw_board(board: [[Option<Piece>; 8]; 8]) {
        for column in board.iter() {
            for piece in column {
                match piece {
                    Some(_piece) => {
                            match _piece.piece_type {
                                PieceType::Knight => print!("Kn "),
                                _ => print!("{}  ", format!("{:?}", _piece.piece_type).get(0..1).unwrap())
                            }
                        },
                    None => print!("-  ")
                }
            }

            print!("\n");
        }
    }

    #[test]
    fn r#const() {
        let game = Game::new();

        draw_board(game.get_board());

        assert_eq!(game.get_turn(), Colour::White);
    }

    #[test]
    fn get_allowed_moves_pawn() {
        let mut game = Game::new();

        let allowed_moves = game.get_allowed_moves(0, 1);

        assert_eq!(allowed_moves.len(), 2);

        let move_one = allowed_moves[0];
        let move_two = allowed_moves[1];

        assert_eq!(move_one.0, MoveType::Normal);
        assert_eq!(move_one.1, 0);
        assert_eq!(move_one.2, 3);

        assert_eq!(move_two.0, MoveType::Normal);
        assert_eq!(move_two.1, 0);
        assert_eq!(move_two.2, 2);
    }

    #[test]
    fn get_allowed_moves_rook() {
        let mut game = Game::new();

        let allowed_moves = game.get_allowed_moves(0, 0);

        assert_eq!(allowed_moves.len(), 0);
    }
}