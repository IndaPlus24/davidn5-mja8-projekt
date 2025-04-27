use std::time::Instant;

use crate::{Game, Piece, ROTATION_180, ROTATION_CCW, ROTATION_CW};

use crate::config::input_config::*;

impl Game {
    pub fn handle_game_inputs(&mut self, ctx: &ggez::Context) {
        let keyboard = &ctx.keyboard;

        // Move left
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::MoveLeft).unwrap()) {
            self.move_piece(-1, 0);
            self.moving_left = true;
            self.moving_right = false;
            self.das_charged = false;
            self.das_start = Some(Instant::now());
            self.arr_start = None;
        }
        if keyboard.is_key_just_released(*self.controls.get(&GameAction::MoveLeft).unwrap()) {
            if !self.moving_right {
                self.moving_left = false;
                self.das_charged = false;
                self.das_start = None;
                self.arr_start = None;
            }
        }

        // Move right
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::MoveRight).unwrap()) {
            self.move_piece(1, 0);
            self.moving_right = true;
            self.moving_left = false;
            self.das_charged = false;
            self.das_start = Some(Instant::now());
            self.arr_start = None;
        }
        if keyboard.is_key_just_released(*self.controls.get(&GameAction::MoveRight).unwrap()) {
            if !self.moving_left {
                self.moving_right = false;
                self.das_charged = false;
                self.das_start = None;
                self.arr_start = None;
            }
        }

        // Soft drop
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::SoftDrop).unwrap()) {
            self.last_drop = Instant::now();
            self.set_gravity_soft(self.gravity + self.sds);
        }
        if keyboard.is_key_just_released(*self.controls.get(&GameAction::SoftDrop).unwrap()) {
            self.set_gravity_soft(self.gravity);
        }

        // Hard drop
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HardDrop).unwrap()) {
            self.hard_drop();
        }

        // Rotation handling
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::RotateCw).unwrap()) {
            println!("Rotating CW...");
            self.rotate(ROTATION_CW);
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::RotateCcw).unwrap()) {
            println!("Rotating CCW...");
            self.rotate(ROTATION_CCW);
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::Rotate180).unwrap()) {
            println!("Rotating 180...");
            self.rotate(ROTATION_180);
        }

        // Hold
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::Hold).unwrap())
            && self.can_hold
        {
            println!("Holding Piece");

            let mut held_piece = self.active_piece.clone();
            held_piece.midpoint = (20, 4);
            held_piece.rotation = 0;
            let blocks: Vec<(isize, isize)> =
                Piece::get_block_positions(self.active_piece.piece_type, 0);
            held_piece.block_positions = blocks;

            match self.held_piece.take() {
                Some(previous_held) => {
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
