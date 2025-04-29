use std::{collections::{HashSet, VecDeque}, time::Duration};

use ggez::Context;
use rand::Rng;

use crate::{board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS}, Game, PieceType, ROTATION_CCW, ROTATION_CW};
use super::{bot_input::BotInput, move_outcome::{MoveOutcome, MovementState}};

#[derive(Clone)]
pub struct Bot {
    pub game : Game,
    pub inputs : Vec<BotInput>,
    pub fitness : f64,
    pub weights : [f64 ; 4],
    pub game_steps : i32,
}

impl Bot{

    pub fn new() -> Self {
        Self{
            game: Game::new(),
            inputs: Vec::new(),
            fitness: 0.,
            weights: [
                //Placeholder values from https://github.com/takado8/Tetris
                -0.798752914564018,
                0.522287506868767,
                -0.24921408023878,
                -0.164626498034284
            ],
            game_steps: 0,
        }
    }

    pub fn get_best_move_sequence(&mut self) -> Vec<BotInput>{
        let all_outcomes = self.get_all_move_outcomes();
        self.evaluate_move_outcomes(all_outcomes).move_sequence
    }

    pub fn evaluate_move_outcomes(&self, outcomes: Vec<MoveOutcome>) -> MoveOutcome {
        let mut evaluation: Vec<f64> = Vec::new();

        let weights = self.weights;
        for outcome in &outcomes {
            evaluation.push(
                outcome.aggregate_height as f64 * weights[0]
                    + outcome.lines_cleared as f64 * weights[1]
                    + outcome.holes as f64 * weights[2]
                    + outcome.bumpiness as f64 * weights[3]
            );
        }
        let best_index = evaluation
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap_or(0);

        outcomes[best_index].clone()
    }

    pub fn get_all_move_outcomes(&mut self) -> Vec<MoveOutcome> {
        let mut final_states: Vec<MoveOutcome> = Vec::new();
        let mut visited: HashSet<((isize,isize),usize)> = HashSet::new();
        let mut queue: VecDeque<MovementState> = VecDeque::new();
        let mut visited_board_states: HashSet<
            [[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
        > = HashSet::new();
        let moves: Vec<(i32, i32)> = vec![(0, -1), (0, 1),(-1, 0)]; //left right down

        let start_state = MovementState {
            game : self.game.clone(),
            moves_so_far: Vec::new(),
        };

        queue.push_back(start_state.clone());
        let p = start_state.game.active_piece;
        visited.insert(((p.midpoint.0, p.midpoint.1), p.rotation));

        while let Some(mut current_state) = queue.pop_front() {
            //Move Right Left and Down
            for (dr, dc) in &moves {
                let mut game_clone = current_state.game.clone();

                //If its possible to move the piece in the direction
                if game_clone.move_piece(*dc as isize,*dr as isize) {

                    let mut new_moves = current_state.moves_so_far.clone();
                    new_moves.push(match (dr, dc) {
                        (-1, 0) => BotInput::MoveDown,
                        (0, -1) => BotInput::MoveLeft,
                        (0, 1) => BotInput::MoveRight,
                        _ => unreachable!(),
                    });

                    let new_state = MovementState {
                        game : game_clone,
                        moves_so_far: new_moves,
                    };

                    let p = &new_state.game.active_piece;
                    if !visited.contains(&((p.midpoint.0,p.midpoint.1), p.rotation)) {
                        visited.insert(((p.midpoint.0,p.midpoint.1), p.rotation));
                        queue.push_back(new_state.clone());
                        // These Piece types only have 2 "types" of rotations
                        if new_state.game.active_piece.piece_type == PieceType::Z
                        || new_state.game.active_piece.piece_type == PieceType::S
                        || new_state.game.active_piece.piece_type == PieceType::I{
                            visited.insert(((new_state.game.active_piece.midpoint), (new_state.game.active_piece.rotation + 2) % 4));
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
                                aggregate_height: Game::get_aggregate_height(&current_state.game.board),
                                holes: Game::count_holes(&current_state.game.board),
                                bumpiness: Game::count_bumpiness(&current_state.game.board),
                                is_t_spin: false, // TODO CHECK IF MOVE IS A T SPIN,
                                move_sequence: moves,
                            });
                        }
                    }
                }
            }

            // Rotations
            if current_state.game.active_piece.piece_type != PieceType::O {
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
                            new_moves.push(BotInput::MoveDown);

                            let new_state = MovementState {
                                game : game_clone,
                                moves_so_far: new_moves,
                            };

                            let p = &new_state.game.active_piece;
                        if !visited.contains(&((p.midpoint.0,p.midpoint.1), p.rotation)){
                            visited.insert(((p.midpoint.0,p.midpoint.1), p.rotation));
                            queue.push_back(new_state.clone());


                            // These Piece types only have 2 "types" of rotations
                            if new_state.game.active_piece.piece_type == PieceType::Z
                            || new_state.game.active_piece.piece_type == PieceType::S
                            || new_state.game.active_piece.piece_type == PieceType::I
                                {
                                    visited.insert(((new_state.game.active_piece.midpoint), (new_state.game.active_piece.rotation + 2) % 4));
                                }
                            }
                        }
                    }
                }
            }
        }
        final_states
    }

    pub fn random_weights() -> [f64 ; 4] {
        let mut rng = rand::rng();
        [rng.random_range(-1.0..1.0);4 ]
    }

    pub fn crossover(parent1: &Bot, parent2: &Bot) -> Bot {
        let mut child = Bot::new();
        for i in 0..child.weights.len() {
            child.weights[i] = if rand::random() {
                parent1.weights[i]
            } else {
                parent2.weights[i]
            };
        }

        child
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::rng();
        for weight in &mut self.weights.iter_mut() {
            if rng.random::<f32>() < MUTATION_RATE {
                *weight += rng.random_range(-0.1..0.1);
            }
        }
    }

    pub fn compute_fitness(&self) -> f64 {
        let lines_cleared = self.game.score as f64;
        let survival_bonus = self.game_steps as f64 * 0.01; // small reward for lasting long
        let height_penalty = Game::get_aggregate_height(&self.game.board) as f64 * 0.5;

        lines_cleared * 10.0 + survival_bonus - height_penalty
    }

    pub fn render_bot_game(&mut self, ctx: &mut Context) {
        if self.inputs.len() == 0 {
            self.inputs = self.get_best_move_sequence();
        }

        if self.game.game_over{
            return;
        }


        // IF THE TICK COUNT MATCHES THE CURRENT LEVELS TICK COUNT
        if self.game.last_drop.elapsed() >= self.game.fall_timing{
            self.game.last_drop += self.game.fall_timing;

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
                        self.game.score += 1;
                        self.game.place_piece();
                    }
                }
            }
        }
    }
}
const MUTATION_RATE: f32 = 0.05;
