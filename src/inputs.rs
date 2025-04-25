use crate::consts::{
    GameState, ARR_TICKS, DAS_TICKS, TICKS_BEFORE_NEXT_PIECE, TICKS_BETWEEN_INPUTS,
    TICKS_BETWEEN_ROTATIONS,
};
use crate::{Game, Piece, ROTATION_180, ROTATION_CCW, ROTATION_CW};

use crate::config::input_config::*;

impl Game {
    pub fn handle_game_inputs(&mut self, ctx: &ggez::Context) {
        let keyboard = &ctx.keyboard;
        let dt = ctx.time.delta().as_secs_f32();

        let right_held =
            keyboard.is_key_pressed(*self.controls.get(&GameAction::MoveRight).unwrap());
        let left_held = keyboard.is_key_pressed(*self.controls.get(&GameAction::MoveLeft).unwrap());

        let direction = if right_held && !left_held {
            Some(1)
        } else if left_held && !right_held {
            Some(-1)
        } else {
            None
        };
        match direction {
            Some(dir) => {
                if self.das_direction != Some(dir) {
                    // New direction pressed: move once and start DAS
                    self.move_piece(dir, 0);
                    self.das_direction = Some(dir);
                    self.das_timer = 0.;
                    self.arr_timer = 0.;
                } else {
                    self.das_timer += dt;
                    if self.das_timer >= DAS_TICKS {
                        self.arr_timer += dt;
                        if self.arr_timer >= ARR_TICKS {
                            self.move_piece(dir, 0);
                            self.arr_timer = 0.;
                        }
                    }
                }
            }
            None => {
                // No direction held
                self.das_direction = None;
                self.das_timer = 0.;
                self.arr_timer = 0.;
            }
        }

        if keyboard.is_key_pressed(*self.controls.get(&GameAction::SoftDrop).unwrap())
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.move_piece(0, -1);
            self.ticks_since_last_input = 0.;
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HardDrop).unwrap()) {
            self.hard_drop();
            self.ticks_since_last_input = 0.;
            //Spawn new piece immedietly
            self.ticks_without_moving_down = TICKS_BEFORE_NEXT_PIECE + 1.;
            self.check_full_line();
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::RotateCw).unwrap())
            && self.ticks_since_last_rotation > TICKS_BETWEEN_ROTATIONS
        {
            println!("Rotating CW...");
            self.rotate(ROTATION_CW);
            self.ticks_since_last_rotation = 0.;
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::RotateCcw).unwrap())
            && self.ticks_since_last_rotation > TICKS_BETWEEN_ROTATIONS
        {
            println!("Rotating CCW...");
            self.rotate(ROTATION_CCW);
            self.ticks_since_last_rotation = 0.;
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::Rotate180).unwrap())
            && self.ticks_since_last_rotation > TICKS_BETWEEN_ROTATIONS
        {
            println!("Rotating 180...");
            self.rotate(ROTATION_180);
            self.ticks_since_last_rotation = 0.;
        }

        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HoldPiece).unwrap())
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

    pub fn handle_start_screen_inputs(&mut self, ctx: &ggez::Context) {
        let keyboard = &ctx.keyboard;
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HardDrop).unwrap()) {
            self.game_state = GameState::MainMenu;
        }
    }

    pub fn handle_main_menu_inputs(&mut self, ctx: &ggez::Context) {
        let keyboard = &ctx.keyboard;
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::SoftDrop).unwrap()) {
            self.animation_state.selected_item_main_menu = 1
        } else if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::Up).unwrap()) {
            self.animation_state.selected_item_main_menu = 0;
        } else if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HardDrop).unwrap()) {
            self.game_state = if self.animation_state.selected_item_main_menu == 0 {
                GameState::GameModeSelector
            } else {
                GameState::Settings
            };
        }
    }

    pub fn handle_gamemode_selector_inputs(&mut self, ctx: &ggez::Context) {
        let keyboard = &ctx.keyboard;
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::SoftDrop).unwrap()) {
            self.animation_state.selected_item_gamemode_selector =
                if self.animation_state.selected_item_gamemode_selector == 2 {
                    2
                } else {
                    self.animation_state.selected_item_gamemode_selector + 1
                };
        } else if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::Up).unwrap()) {
            self.animation_state.selected_item_gamemode_selector =
                if self.animation_state.selected_item_gamemode_selector == 0 {
                    0
                } else {
                    self.animation_state.selected_item_gamemode_selector - 1
                };
        } else if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HardDrop).unwrap()) {
            match self.animation_state.selected_item_gamemode_selector {
                0 => self.game_state = GameState::Multiplayer,
                1 => self.game_state = GameState::Singleplayer,
                _ => self.game_state = GameState::BotSelector,
            }
        }
    }

    pub fn handle_bot_selector_inputs(&mut self, ctx: &ggez::Context) {
        let keyboard = &ctx.keyboard;
        if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::SoftDrop).unwrap()) {
            self.animation_state.selected_item_bot_selector =
                if self.animation_state.selected_item_bot_selector == 2 {
                    2
                } else {
                    self.animation_state.selected_item_bot_selector + 1
                };
        } else if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::Up).unwrap()) {
            self.animation_state.selected_item_bot_selector =
                if self.animation_state.selected_item_bot_selector == 0 {
                    0
                } else {
                    self.animation_state.selected_item_bot_selector - 1
                };
        } else if keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HardDrop).unwrap()) {
            match self.animation_state.selected_item_bot_selector {
                0 => self.game_state = GameState::Multiplayer,
                1 => self.game_state = GameState::Singleplayer,
                _ => self.game_state = GameState::VsBots,
            }
        }
    }
}
