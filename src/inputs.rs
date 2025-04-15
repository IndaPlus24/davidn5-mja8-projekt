use crate::{AppState, HOLD_PIECE, HOLD_PIECE_MIDDLE, MOVE_PIECE_DOWN_HARD_DROP, MOVE_PIECE_DOWN_SOFT_DROP, MOVE_PIECE_LEFT, MOVE_PIECE_RIGHT, ROTATE_PIECE_180, ROTATE_PIECE_CCW, ROTATE_PIECE_CW, ROTATION_180, ROTATION_CCW, ROTATION_CW, TICKS_BEFORE_NEXT_PIECE, TICKS_BETWEEN_INPUTS, TICKS_BETWEEN_ROTATIONS};

impl AppState {
    pub fn handle_inputs(&mut self, ctx : &ggez::Context){

        let keyboard = &ctx.keyboard;

        if keyboard.is_key_pressed(MOVE_PIECE_RIGHT)
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.board.move_piece(&mut self.active_piece, 1, 0);
            self.ticks_since_last_input = 0;
        }

        if keyboard.is_key_pressed(MOVE_PIECE_LEFT)
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.board.move_piece(&mut self.active_piece, -1, 0);
            self.ticks_since_last_input = 0;
        }

        if keyboard.is_key_pressed(MOVE_PIECE_DOWN_SOFT_DROP)
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.board.move_piece(&mut self.active_piece, 0, 1);
            self.ticks_since_last_input = 0;
        }

        if keyboard.is_key_just_pressed(MOVE_PIECE_DOWN_HARD_DROP) {
            self.board.hard_drop(&mut self.active_piece);
            self.ticks_since_last_input = 0;
            //SPAWN A NEW PIECE IMMEDIETLY
            self.ticks_without_moving_down = TICKS_BEFORE_NEXT_PIECE;
            self.board.check_full_line();
        }

        if keyboard.is_key_just_pressed(ROTATE_PIECE_CW)
            && self.ticks_since_last_rotation > TICKS_BETWEEN_ROTATIONS
        {
            println!("Rotating CW...");
            self.board.rotate(&mut self.active_piece, ROTATION_CW);
            self.ticks_since_last_rotation = 0;
        }

        if keyboard.is_key_just_pressed(ROTATE_PIECE_CCW)
            && self.ticks_since_last_rotation > TICKS_BETWEEN_ROTATIONS
        {
            println!("Rotating CCW...");
            self.board.rotate(&mut self.active_piece, ROTATION_CCW);
            self.ticks_since_last_rotation = 0;
        }

        if keyboard.is_key_just_pressed(ROTATE_PIECE_180)
            && self.ticks_since_last_rotation > TICKS_BETWEEN_ROTATIONS
        {
            println!("Rotating 180...");
            self.board.rotate(&mut self.active_piece, ROTATION_180);
            self.ticks_since_last_rotation = 0;
        }

        if keyboard.is_key_just_pressed(HOLD_PIECE) && self.can_hold {
            println!("Holding Piece"); 
        
            let mut held_piece = self.active_piece.clone();
            held_piece.midpoint = HOLD_PIECE_MIDDLE;
            
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