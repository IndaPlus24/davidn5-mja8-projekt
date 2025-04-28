#[cfg(test)]
mod tests {
    use crate::{
        board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS},
        bots::bot::Bot,
        Game, Piece, PieceType,
    };

    #[test]
    #[allow(non_snake_case)]
    fn test_find_all_move_outcomes_piece_O() {
        let mut bot = Bot {
            game: Game::new(),
            inputs: Vec::new(),
            fitness: 0.,
            weights: Vec::new(),
            game_steps: 0,
        };

        bot.game.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        bot.game.active_piece = Piece::new(PieceType::O, 0);

        let outcomes = bot.get_all_move_outcomes();

        assert_eq!(
            outcomes.len(),
            9,
            "Expected exactly 9 move outcomes, found {}",
            outcomes.len()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_find_all_move_outcomes_piece_I() {
        let mut bot = Bot {
            game: Game::new(),
            inputs: Vec::new(),
            fitness: 0.,
            weights: Vec::new(),
            game_steps: 0,
        };

        bot.game.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        bot.game.active_piece = Piece::new(PieceType::I, 0);

        let outcomes = bot.get_all_move_outcomes();

        assert_eq!(
            outcomes.len(),
            17,
            "Expected exactly 17 move outcomes, found {}",
            outcomes.len()
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_find_all_move_outcomes_piece_L() {
        let mut bot = Bot {
            game: Game::new(),
            inputs: Vec::new(),
            fitness: 0.,
            weights: Vec::new(),
            game_steps: 0,
        };

        bot.game.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        bot.game.active_piece = Piece::new(PieceType::L, 0);

        let outcomes = bot.get_all_move_outcomes();

        assert_eq!(
            outcomes.len(),
            34,
            "Expected exactly 34 move outcomes, found {}",
            outcomes.len()
        );
    }
}