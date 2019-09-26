extern crate nilslofchess;

#[cfg(test)]
mod game {
    use nilslofchess::Game;
    use nilslofchess::Colour;
    use nilslofchess::Piece;
    use nilslofchess::PieceType;

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
}