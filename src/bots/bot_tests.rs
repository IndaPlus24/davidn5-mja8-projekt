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
        let mut bot = Bot::new();

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
        let mut bot = Bot::new();

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
        let mut bot = Bot::new();

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

    #[test]
    #[allow(non_snake_case)]
    fn test_find_best_move() {
        let mut bot = Bot::new();

        bot.game.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        bot.game.board[0] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[1] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[2] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[3] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.active_piece = Piece::new(PieceType::I, 0);

        let move_outcome = bot.get_all_move_outcomes();
        let best_outcome = bot.evaluate_move_outcomes(move_outcome);

        assert_eq!(best_outcome.0.lines_cleared, 4.0);
    }

    #[test]
    #[allow(non_snake_case)]
    pub fn test_aggregate_height() {
        let mut bot = Bot::new();

        bot.game.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        bot.game.board[0] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[1] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[2] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[3] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];

        let h = Game::get_aggregate_height(&bot.game.board);

        assert_eq!(h, 36.0);
    }

    #[test]
    #[allow(non_snake_case)]
    pub fn test_holes() {
        let mut bot = Bot::new();

        bot.game.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        bot.game.board[0] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[1] = [
            None,
            Some(PieceType::I),
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[2] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[3] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];

        let h = Game::count_holes(&bot.game.board);

        assert_eq!(h, 2.0);
    }

    #[test]
    #[allow(non_snake_case)]
    pub fn test_get_bumpiness() {
        let mut bot = Bot::new();

        bot.game.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        bot.game.board[0] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[1] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[2] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[3] = [
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];

        let h = Game::count_bumpiness(&bot.game.board);

        assert_eq!(h, 4.0);
    }

    #[test]
    #[allow(non_snake_case)]
    pub fn test_best_move_piece_L() {
        let mut bot = Bot::new();
        

        bot.game.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        bot.game.board[0] = [
            Some(PieceType::I),
            Some(PieceType::I),
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[1] = [
            Some(PieceType::I),
            Some(PieceType::I),
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.active_piece = Piece::new(PieceType::L, 0);

        let move_outcome = bot.get_all_move_outcomes();
        let best_outcome = bot.evaluate_move_outcomes(move_outcome);

        println!("{:?}", best_outcome.0.move_sequence);
        assert_eq!(best_outcome.0.lines_cleared, 2.0);
    }

    #[test]
    #[allow(non_snake_case)]
    pub fn test_best_move_piece_T() {
        let mut bot = Bot::new();
        bot.game.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        bot.game.board[0] = [
            Some(PieceType::I),
            Some(PieceType::I),
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.board[1] = [
            Some(PieceType::I),
            Some(PieceType::I),
            None,
            None,
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
            Some(PieceType::I),
        ];
        bot.game.active_piece = Piece::new(PieceType::T, 0);

        let move_outcome = bot.get_all_move_outcomes();
        let best_outcome = bot.evaluate_move_outcomes(move_outcome);

        println!("{:?}", best_outcome.0.move_sequence);
        assert_eq!(best_outcome.0.lines_cleared, 2.0);
    }
}
