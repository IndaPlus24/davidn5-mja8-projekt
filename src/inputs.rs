use crate::{AppState, Piece, HOLD_PIECE_MIDDLE, ROTATION_180, ROTATION_CCW, ROTATION_CW, TICKS_BEFORE_NEXT_PIECE, TICKS_BETWEEN_INPUTS, TICKS_BETWEEN_ROTATIONS};

use crate::config::input_config::*;

impl AppState {
    pub fn handle_inputs(&mut self, ctx : &ggez::Context){

        let keyboard = &ctx.keyboard;

        if keyboard.is_key_pressed(*self.controls.get(&GameAction::MoveRight).unwrap())
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.board.move_piece(&mut self.active_piece, 1, 0);
            self.ticks_since_last_input = 0;
        }

        if keyboard.is_key_pressed(*self.controls.get(&GameAction::MoveLeft).unwrap())
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.board.move_piece(&mut self.active_piece, -1, 0);
            self.ticks_since_last_input = 0;
        }

        if keyboard.is_key_pressed(*self.controls.get(&GameAction::SoftDrop).unwrap())
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.board.move_piece(&mut self.active_piece, 0, 1);
            self.ticks_since_last_input = 0;
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HardDrop).unwrap()) {
            self.board.hard_drop(&mut self.active_piece);
            self.ticks_since_last_input = 0;
            //Spawn new piece immedietly
            self.ticks_without_moving_down = TICKS_BEFORE_NEXT_PIECE;
            self.board.check_full_line();
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::RotateCw).unwrap())
            && self.ticks_since_last_rotation > TICKS_BETWEEN_ROTATIONS
        {
            println!("Rotating CW...");
            self.board.rotate(&mut self.active_piece, ROTATION_CW);
            self.ticks_since_last_rotation = 0;
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::RotateCcw).unwrap())
            && self.ticks_since_last_rotation > TICKS_BETWEEN_ROTATIONS
        {
            println!("Rotating CCW...");
            self.board.rotate(&mut self.active_piece, ROTATION_CCW);
            self.ticks_since_last_rotation = 0;
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::Rotate180).unwrap())
            && self.ticks_since_last_rotation > TICKS_BETWEEN_ROTATIONS
        {
            println!("Rotating 180...");
            self.board.rotate(&mut self.active_piece, ROTATION_180);
            self.ticks_since_last_rotation = 0;
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HoldPiece).unwrap()) && self.can_hold {
            println!("Holding Piece"); 
        
            let mut held_piece = self.active_piece.clone();
            held_piece.midpoint = HOLD_PIECE_MIDDLE;
            held_piece.rotation = 0;
            let blocks: Vec<(isize, isize)> = Piece::get_block_positions(self.active_piece.piece_type, 0    );
            held_piece.block_positions = blocks;
            
            match self.held_piece.take() {
                Some(mut previous_held) => {
                    previous_held.midpoint = (-1,4);
                    self.active_piece = previous_held;
                }
                None => {
                    self.spawn_new_piece();
                }
            }
        
            self.held_piece = Some(held_piece);
            self.can_hold = false;

        }
    }
}