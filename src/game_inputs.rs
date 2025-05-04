use std::time::Instant;

use crate::{Game, ROTATION_180, ROTATION_CCW, ROTATION_CW};

use crate::config::input_config::*;

impl Game {
    pub fn handle_game_inputs(&mut self, ctx: &ggez::Context) {
        let keyboard = &ctx.keyboard;

        // Move left
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::MoveLeft).unwrap()) {
            if self.move_piece(-1, 0) {self.add_action()}
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
            if self.move_piece(1, 0) {self.add_action()}
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
            self.soft_dropping = true;
            self.last_drop = Instant::now();
            self.set_gravity_soft(self.gravity + self.sds);
        }
        if keyboard.is_key_just_released(*self.controls.get(&GameAction::SoftDrop).unwrap()) {
            self.soft_dropping = false;
            self.set_gravity_soft(self.gravity);
        }

        // Hard drop
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HardDrop).unwrap()) {
            self.hard_drop();
        }

        // Rotation handling
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::RotateCw).unwrap()) {
            self.rotate(ROTATION_CW);
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::RotateCcw).unwrap()) {
            self.rotate(ROTATION_CCW);
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::Rotate180).unwrap()) {
            self.rotate(ROTATION_180);
        }

        // Hold
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::Hold).unwrap())
            && self.can_hold
        {
            let held_piece = self.active_piece.piece_type;

            if let Some(current_held) = self.held_piece {
                self.spawn_piece(current_held);
            } else {
                self.spawn_piece_from_queue();
            }

            self.held_piece = Some(held_piece);
            self.can_hold = false;
        }

        // Reset
        if keyboard.is_key_just_pressed(ggez::input::keyboard::KeyCode::R) {
            self.reset_game();
        }
    }
}
