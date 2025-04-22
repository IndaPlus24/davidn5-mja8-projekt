use std::collections::{HashMap, VecDeque};

use ggez::Context;

use crate::board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS};
use crate::consts::{LEVELS_GRAVITY_THRESHOLD, TICKS_BEFORE_NEXT_PIECE};
use crate::{default_keyboard_keybindings, GameAction, KeyCode, Piece, PieceType};

pub struct Game {
    pub board: [[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    pub game_over: bool,
    pub score : usize, 
    pub gravity_timer: f32,
    pub current_level: usize,
    pub active_piece: Piece,
    pub held_piece: Option<Piece>,
    pub piece_queue: VecDeque<Piece>,
    pub ticks_since_last_input: f32,
    pub ticks_since_last_rotation: f32,
    pub ticks_without_moving_down: f32,
    pub can_hold: bool,
    pub controls: HashMap<GameAction, KeyCode>,
    pub das_direction: Option<isize>,
    pub das_timer: f32,
    pub arr_timer: f32,
}

impl Game {
    pub fn new() -> Self {
        let mut piece_queue: VecDeque<Piece> = VecDeque::new();
        let l = PieceType::get_random_as_list();
        for p in l {
            piece_queue.push_back(Piece::new(p, 0));
        }

        let active_piece = piece_queue.pop_front().unwrap();

        Game {
            board: [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
            game_over: false,
            score : 0,
            gravity_timer: 0.,
            current_level: 0,
            active_piece,
            held_piece: None,
            piece_queue,
            ticks_since_last_input: 0.,
            ticks_since_last_rotation: 0.,
            ticks_without_moving_down: 0.,
            can_hold: true,
            controls: default_keyboard_keybindings(),
            das_direction: None,
            das_timer: 0.,
            arr_timer: 0.,
        }
    }

    pub fn spawn_new_piece(&mut self) {
        if self.piece_queue.len() < 7 {
            //7-bag
            let l = PieceType::get_random_as_list();
            for p in l {
                self.piece_queue.push_back(Piece::new(p, 0));
            }
        }

        println!("Spawning new piece...");
        self.active_piece = self.piece_queue.pop_front().unwrap();

        if self.check_game_over() {
            println!("Game Over!");
            self.game_over = true;
        }

        self.ticks_without_moving_down = 0.;
        self.can_hold = true;
    }

    pub fn check_game_over(&mut self) -> bool {
        let piece = &self.active_piece;
        let (mr, mc) = self.active_piece.midpoint;
        piece.block_positions.iter().any(|(dr, dc)| {
            let r = mr + dr;
            let c = mc + dc;

            // If the active piece is outside the board
            if r < 0
                || r >= BOARD_AMOUNT_ROWS as isize
                || c < 0
                || c >= BOARD_AMOUNT_COLUMNS as isize
            {

                return true;
            }

            // If the active piece overlaps with another piece.
            self.board[r as usize][c as usize].is_some()
        })
    }

    pub fn next_tick(&mut self, ctx: &mut Context) {
        if self.game_over {
            return;
        }

        let dt = ctx.time.delta().as_secs_f32();

        self.gravity_timer += dt;
        self.ticks_since_last_input += dt;
        self.ticks_since_last_rotation += dt;

        //Spawn new Piece
        if self.ticks_without_moving_down > TICKS_BEFORE_NEXT_PIECE {
            self.spawn_new_piece();
        }

        //Handle inputs
        self.handle_inputs(ctx);

        // IF THE TICK COUNT MATCHES THE CURRENT LEVELS TICK COUNT
        if self.gravity_timer > LEVELS_GRAVITY_THRESHOLD[self.current_level] {
            self.gravity_timer = 0.;

            if !self.move_piece(0, -1) {
                self.ticks_without_moving_down += dt;
                self.place_piece();
                println!("Piece at bottom...");
                println!("Checking Lines...");
                self.check_full_line();
                self.spawn_new_piece();
            }
        }
    }
}
