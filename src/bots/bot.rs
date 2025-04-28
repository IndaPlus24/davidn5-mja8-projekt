use std::collections::{HashSet, VecDeque};

use crate::{board::{self, BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS}, Game, Piece, PieceType, ROTATION_180, ROTATION_CCW, ROTATION_CW};
use super::{bot_input::BotInput, move_outcome::{MoveOutcome, MovementState}};

pub struct Bot {
    pub game : Game,
    pub inputs : Vec<BotInput>,
    pub fitness : f64,
    pub weights : Vec<f64>,
    pub game_steps : i32,
}

impl Bot{
    pub fn get_all_move_outcomes(&mut self) -> Vec<MoveOutcome> {
        let mut final_states: Vec<MoveOutcome> = Vec::new();
        let mut visited: HashSet<((isize,isize),usize)> = HashSet::new();
        let mut queue: VecDeque<MovementState> = VecDeque::new();
        let mut visited_board_states: HashSet<
            [[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
        > = HashSet::new();
        let moves: Vec<(i32, i32)> = vec![(-1, 0), (0, -1), (0, 1)]; // down left right

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

                            println!("{:?}, {}", current_state.game.active_piece.midpoint, current_state.game.active_piece.rotation);
                            
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
}