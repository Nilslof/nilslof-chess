extern crate nilslofchess;

#[cfg(test)]
mod game {
    use nilslofchess::Game;
    use nilslofchess::Colour;
    use nilslofchess::Piece;
    use nilslofchess::PieceType;
    use nilslofchess::MoveType;

    use colored::*;

    fn draw_board(board: [[Option<Piece>; 8]; 8]) {
        for column in board.iter() {
            for piece in column {
                match piece {
                    Some(_piece) => {
                            match _piece.get_colour() {
                                Colour::White => {
                                    match _piece.piece_type {
                                        PieceType::Knight => print!("{} ", "Kn".on_white().black()),
                                        _ => print!("{}  ", format!("{:?}", _piece.piece_type).get(0..1).unwrap().on_white().black())
                                    }
                                },
                                _ => {
                                    match _piece.piece_type {
                                        PieceType::Knight => print!("Kn "),
                                        _ => print!("{}  ", format!("{:?}", _piece.piece_type).get(0..1).unwrap())
                                    }
                                }
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

        let file = 0;
        let rank = 1;

        assert_eq!(game.get_board()[file][rank].unwrap().get_colour(), Colour::White);

        let allowed_moves = game.get_allowed_moves(file, rank);

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

        let file = 0;
        let rank = 0;

        assert_eq!(game.get_board()[file][rank].unwrap().get_colour(), Colour::White);

        let allowed_moves = game.get_allowed_moves(file, rank);

        println!("Initial amount of allowed moves by Rook: {}", allowed_moves.len());
        for i in 0..allowed_moves.len() {
            println!("    - Move 1: [ type: {:?}, file: {}, rank: {} ]", allowed_moves[i].0, allowed_moves[i].1, allowed_moves[i].2);
        }

        assert_eq!(allowed_moves.len(), 0);
    }
}