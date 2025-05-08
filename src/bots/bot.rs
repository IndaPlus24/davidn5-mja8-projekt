use std::{
    collections::{HashSet, VecDeque}, time::{Duration, Instant}
};

use ggez::Context;
use rand::Rng;

use super::{
    bot_input::BotInput,
    move_outcome::{MoveOutcome, MovementState},
};
use crate::{
    board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS}, consts::{BOT_DIFFICULTY_SPEEDS, GAME_1_SOLO_POS, GAME_1_SOLO_SCL}, Game, PieceType, ROTATION_CCW, ROTATION_CW
};

#[derive(Clone)]
pub struct Bot {
    pub game: Game,
    pub inputs: Vec<BotInput>,
    pub fitness: f64,
    pub weights: [f64; 4],
    pub game_steps: i32,
    pub difficulty: usize, // ranges for 0 - 2
}

impl Bot {
    pub fn new(difficulty: usize) -> Self {

        let mut g = Game::new(GAME_1_SOLO_POS, GAME_1_SOLO_SCL);
        g.reset_game();

        Self {
            game: g,
            inputs: Vec::new(),
            fitness: 0.,
            /*
                aggregate height 
                lines cleared
                holes 
                bumpiness
            */
            weights: [
                -0.610066,
                0.760666,
                -0.35663,
                -0.184483
            ],
            game_steps: 0,
            difficulty,
        }
    }

    pub fn with_random_unit_weights() -> Self {
        let mut w = Bot::random_weights();
        Self::normalize(&mut w);
        Self {
            weights: w,
            fitness: 0.0,
            game: Game::new(GAME_1_SOLO_POS, GAME_1_SOLO_SCL),
            inputs: vec![],
            game_steps: 0,
            difficulty: 0
        }
    }

    fn normalize(w: &mut [f64; 4]) {
        let norm = w.iter().map(|x| x * x).sum::<f64>().sqrt();
        for i in 0..4 {
            w[i] /= norm;
        }
    }

    pub fn random_crossover(p1: &Bot, p2: &Bot) -> Bot {
        let mut rng = rand::rng();
        let mut w = [0.0; 4];
        for i in 0..4 {
            w[i] = if rng.random_bool(0.5) {
                p1.weights[i]
            } else {
                p2.weights[i]
            };
        }
        Self::normalize(&mut w);
        Self {
            weights: w,
            fitness: 0.0,
            game: Game::new(GAME_1_SOLO_POS, GAME_1_SOLO_SCL),
            inputs: vec![],
            game_steps: 0,
            difficulty: 0,
        }
    }

    pub fn get_best_move_sequence(&mut self) -> Vec<BotInput> {
        let mut bot_clone = self.clone(); 

        let held_piece = bot_clone.game.active_piece.piece_type;

        if let Some(current_held) = self.game.held_piece {
            bot_clone.game.spawn_piece(current_held);
        } else {
            bot_clone.game.spawn_piece_from_queue();
        }

        bot_clone.game.held_piece = Some(held_piece);
        bot_clone.game.can_hold = false;
        
        let alternative_outcomes = bot_clone.get_all_move_outcomes_by_horizontal_drop(); 
        let all_outcomes = self.get_all_move_outcomes_by_horizontal_drop();

        let active_piece_evaluation = self.evaluate_move_outcomes(all_outcomes);
        let alternative_piece_evalution = bot_clone.evaluate_move_outcomes(alternative_outcomes);

        if active_piece_evaluation.1 > alternative_piece_evalution.1 {
            return active_piece_evaluation.0.move_sequence
        }

        let mut moves = alternative_piece_evalution.0.move_sequence; 
        moves.insert(0,BotInput::Hold);

        moves

    }

    pub fn evaluate_move_outcomes(&self, outcomes: Vec<MoveOutcome>) -> (MoveOutcome, f64) {
        let mut evaluation: Vec<f64> = Vec::new();

        let weights = self.weights;
        for outcome in &outcomes {
            evaluation.push(
                outcome.aggregate_height as f64 * weights[0]
                    + outcome.lines_cleared as f64 * weights[1]
                    + outcome.holes as f64 * weights[2]
                    + outcome.bumpiness as f64 * weights[3],
            );
            
            // println!(
            //     "lines: {}, height: {}, holes: {}, bump: {}, score: {}",
            //     outcome.lines_cleared,
            //     outcome.aggregate_height,
            //     outcome.holes,
            //     outcome.bumpiness,
            //     evaluation[evaluation.len() -1 ]
            // );
        }
        let best_index = evaluation
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap_or(0);

        (outcomes[best_index].clone(), evaluation[best_index])
    }

    // LEGACY FUNCTION wasnt able to use this!
    #[allow(unused)]
    pub fn get_all_move_outcomes(&mut self) -> Vec<MoveOutcome> {
        let mut final_states: Vec<MoveOutcome> = Vec::new();
        let mut visited: HashSet<((isize, isize), usize)> = HashSet::new();
        let mut queue: VecDeque<MovementState> = VecDeque::new();
        let mut visited_board_states: HashSet<
            [[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
        > = HashSet::new();
        let moves: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (0, 1)]; // down, left, right

        let start_state = MovementState {
            game: self.game.clone(),
            moves_so_far: Vec::new(),
            rotations: 0,
        };

        queue.push_back(start_state.clone());
        let p = start_state.game.active_piece;
        visited.insert(((p.midpoint.0, p.midpoint.1), p.rotation));

        while let Some(mut current_state) = queue.pop_front() {
            //Move Right Left and Down
            for (dr, dc) in &moves {
                let mut game_clone = current_state.game.clone();

                //If its possible to move the piece in the direction
                if game_clone.move_piece(*dc as isize, *dr as isize) {
                    let mut new_moves = current_state.moves_so_far.clone();
                    new_moves.push(match (dr, dc) {
                        (-1, 0) => BotInput::MoveDown,
                        (0, -1) => BotInput::MoveLeft,
                        (0, 1) => BotInput::MoveRight,
                        _ => unreachable!(),
                    });

                    let new_state = MovementState {
                        game: game_clone,
                        moves_so_far: new_moves,
                        rotations: 0,
                    };

                    let p = &new_state.game.active_piece;
                    if !visited.contains(&((p.midpoint.0, p.midpoint.1), p.rotation)) {
                        visited.insert(((p.midpoint.0, p.midpoint.1), p.rotation));
                        queue.push_back(new_state.clone());
                        // These Piece types only have 2 "types" of rotations
                        if new_state.game.active_piece.piece_type == PieceType::Z
                            || new_state.game.active_piece.piece_type == PieceType::S
                            || new_state.game.active_piece.piece_type == PieceType::I
                        {
                            visited.insert((
                                (new_state.game.active_piece.midpoint),
                                (new_state.game.active_piece.rotation + 2) % 4,
                            ));
                        }
                    }
                } else {
                    //If the Piece cant move down the current state is a final state
                    if *dr == -1 {
                        current_state.game.simulate_place_piece();

                        // Only add this board state if we haven't seen it before
                        if !visited_board_states.contains(&current_state.game.board) {
                            visited_board_states.insert(current_state.game.board.clone());

                            let mut moves = current_state.moves_so_far.clone();

                            moves.push(BotInput::HardDrop);
                            final_states.push(MoveOutcome {
                                lines_cleared: Game::count_lines_cleared(&current_state.game.board),
                                aggregate_height: Game::get_aggregate_height(
                                    &current_state.game.board,
                                ),
                                holes: Game::count_holes(&current_state.game.board),
                                bumpiness: Game::count_bumpiness(&current_state.game.board),
                                move_sequence: moves,
                            });
                        }
                    }
                }
            }

            // Rotations
            if current_state.game.active_piece.piece_type != PieceType::O
                && current_state.rotations < 5
            {
                let rotations = vec![
                    (ROTATION_CW, BotInput::RotateCW),
                    (ROTATION_CCW, BotInput::RotateCCW),
                ];

                {
                    for (r, b) in rotations {
                        let mut game_clone = current_state.game.clone();
                        if game_clone.rotate(r) {
                            let mut new_moves = current_state.moves_so_far.clone();
                            new_moves.push(b);

                            let new_state = MovementState {
                                game: game_clone,
                                moves_so_far: new_moves,
                                rotations: current_state.rotations + 1,
                            };

                            let p = &new_state.game.active_piece;
                            if !visited.contains(&((p.midpoint.0, p.midpoint.1), p.rotation)) {
                                visited.insert(((p.midpoint.0, p.midpoint.1), p.rotation));
                                queue.push_back(new_state.clone());

                                // These Piece types only have 2 "types" of rotations
                                if new_state.game.active_piece.piece_type == PieceType::Z
                                    || new_state.game.active_piece.piece_type == PieceType::S
                                    || new_state.game.active_piece.piece_type == PieceType::I
                                {
                                    visited.insert((
                                        (new_state.game.active_piece.midpoint),
                                        (new_state.game.active_piece.rotation + 2) % 4,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        final_states
    }

    pub fn random_weights() -> [f64; 4] {
        let mut rng = rand::rng();
        [(); 4].map(|_| rng.random::<f64>() * if rng.random_bool(0.5) { -1. } else { 1.0 })
    }

    pub fn mutate(&mut self, mutation_rate: f64) {
        let mut rng = rand::rng();
        for i in 0..4 {
            if rng.random_bool(mutation_rate) {
                self.weights[i] += rng.random_range(-0.2..0.2);
            }
        }
        Self::normalize(&mut self.weights);
    }

    pub fn compute_fitness(&self) -> f64 {
        self.game.lines as f64
    }

    pub fn render_bot_game(&mut self, _ctx: &mut Context) {

        if let Some(start) = self.game.countdown_start{
            let elapsed = start.elapsed(); 
            if elapsed >= self.game.countdown_duration {
                self.game.countdown_start = None
            }
            self.game.last_drop = Instant::now();
            self.game.start_time = Instant::now();
            return;
        }
        if self.inputs.is_empty() {
            self.inputs = self.get_best_move_sequence();

            if self.inputs.is_empty() {
                return;
            }

            self.inputs.reverse();
        }

        if self.game.game_over {
            return;
        }   
        

        if self.game.last_drop.elapsed() >= Duration::from_millis((1000. / BOT_DIFFICULTY_SPEEDS[self.difficulty]) as u64) {
            self.game.last_drop += Duration::from_millis((1000. / 120.) as u64);

            if let Some(input) = self.inputs.pop() {
                match input {
                    BotInput::MoveLeft => {
                        self.game.move_piece(-1, 0);
                    }
                    BotInput::MoveRight => {
                        self.game.move_piece(1, 0);
                    }
                    BotInput::RotateCCW => {
                        self.game.rotate(ROTATION_CCW);
                    }
                    BotInput::RotateCW => {
                        self.game.rotate(ROTATION_CW);
                    }
                    BotInput::MoveDown => {
                        self.game.move_piece(0, -1);
                        self.game.score += 1;
                    }
                    BotInput::HardDrop => {
                        while self.game.move_piece(0, -1) {}
                        self.game.score += 2;
                        self.game.place_piece();
                    }
                    BotInput::Hold => {
                        let held_piece = self.game.active_piece.piece_type;

                        if let Some(current_held) = self.game.held_piece {
                            self.game.spawn_piece(current_held);
                        } else {
                            self.game.spawn_piece_from_queue();
                        }

                        self.game.held_piece = Some(held_piece);
                        self.game.can_hold = false;
                    }
                }
            }
        }
    }

    pub fn run_game_without_ui(&mut self, max_game_steps: i32) -> f64 {
        while self.game_steps < max_game_steps {
            if self.inputs.len() == 0 {
                self.inputs = self.get_best_move_sequence();
                self.game_steps += 1;
                self.inputs.reverse();
            }

            if self.game.game_over {
                break;
            }

            // IF THE TICK COUNT MATCHES THE CURRENT LEVELS TICK COUNT
            while let Some(input) = self.inputs.pop() {
                match input {
                    BotInput::MoveLeft => {
                        self.game.move_piece(-1, 0);
                    }
                    BotInput::MoveRight => {
                        self.game.move_piece(1, 0);
                    }
                    BotInput::RotateCCW => {
                        self.game.rotate(ROTATION_CCW);
                    }
                    BotInput::RotateCW => {
                        self.game.rotate(ROTATION_CW);
                    }
                    BotInput::MoveDown => {
                        self.game.move_piece(0, -1);
                        self.game.score += 1;
                    }
                    BotInput::HardDrop => {
                        while self.game.move_piece(0, -1) {}
                        self.game.score += 2;
                        self.game.place_piece();
                    }
                    BotInput::Hold => {
                        let held_piece = self.game.active_piece.piece_type;

                        if let Some(current_held) = self.game.held_piece {
                            self.game.spawn_piece(current_held);
                        } else {
                            self.game.spawn_piece_from_queue();
                        }

                        self.game.held_piece = Some(held_piece);
                        self.game.can_hold = false;
                    }
                }
            }
        }

        self.fitness = self.compute_fitness();
        self.game.lines as f64
    }

    pub fn get_all_move_outcomes_by_horizontal_drop(&mut self) -> Vec<MoveOutcome> {
        let mut move_outcomes: Vec<MoveOutcome> = Vec::new();
    
        for rotation in 0..4 {
            let mut base_game = self.game.clone();
            let mut base_moves: Vec<BotInput> = Vec::new();
    
            for _ in 0..rotation {
                if base_game.rotate(ROTATION_CW) {
                    base_moves.push(BotInput::RotateCW);
                }
            }
    
            // Move piece all the way to the left
            while base_game.move_piece(-1, 0) {
                base_moves.push(BotInput::MoveLeft);
            }
    
            loop {
                let mut game_cc = base_game.clone();
                let mut moves = base_moves.clone();
    
                // Drop the piece all the way down
                while game_cc.move_piece(0, -1) {}
                moves.push(BotInput::HardDrop);            
                game_cc.place_piece();
    
                move_outcomes.push(MoveOutcome {
                    lines_cleared: Game::count_lines_cleared(&game_cc.board),
                    aggregate_height: Game::get_aggregate_height(&game_cc.board),
                    holes: Game::count_holes(&game_cc.board),
                    bumpiness: Game::count_bumpiness(&game_cc.board),
                    move_sequence: moves,
                });
    
                // Try moving the base piece one column to the right
                if !base_game.move_piece(1, 0) {
                    break;
                }
    
                base_moves.push(BotInput::MoveRight);
            }
        }
    
        move_outcomes
    }
    
}
