use std::collections::{HashMap, VecDeque};
use std::f32::INFINITY;
use std::time::{Duration, Instant};

use ggez::Context;

use crate::board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS};
use crate::consts::{BoardRenderType, LEVEL_GRAVITIES};
use crate::{default_keyboard_keybindings, GameAction, KeyCode, Piece, PieceType};

#[derive(Clone)]
pub struct Game {
    pub board: [[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    pub render_type: BoardRenderType,

    pub game_over: bool,
    pub battle_mode: bool,
    pub garbage_queue: VecDeque<(usize, usize)>, // (amount, column of garbage hole)
    pub active_piece: Piece,
    pub held_piece: Option<Piece>,
    pub piece_queue: VecDeque<Piece>,
    pub can_hold: bool,
    pub controls: HashMap<GameAction, KeyCode>,

    // Timing/movement tomfoolery
    pub moving_right: bool,
    pub moving_left: bool,
    pub soft_dropping: bool,

    pub das: Duration,
    pub das_start: Option<Instant>,
    pub das_charged: bool,
    pub arr: Duration,
    pub arr_start: Option<Instant>,
    pub sds: f32, // Soft drop speed (cells per second)
    pub gravity: f32, // (cells per second)
    pub last_drop: Instant,
    pub fall_timing: Duration, // (time per cell)
    pub on_ground: bool,
    pub on_ground_start: Option<Instant>, // Timer for lock delay
    pub actions_from_ground: usize, // Action counter. If it reaches 15, the piece will automatically lock in place

    // Stats
    pub score: usize,
    pub lines: usize,
    pub level: usize,
    pub pieces: usize,
    pub start_time: Instant,

    // Scoring checks
    pub t_spin: bool,
    pub t_spin_mini: bool,
    pub prev_clear: bool, // true if previous piece resulted in a line clear
    pub combo: usize,
    pub latest_clear_difficult: bool, // true if latest line clear was a tetris or t-spin
    pub back_to_back: bool,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
            render_type: BoardRenderType::Marathon,

            game_over: false,
            battle_mode: false,
            garbage_queue: VecDeque::new(),
            active_piece: Piece::new(PieceType::Z, 0),
            held_piece: None,
            piece_queue: VecDeque::new(),
            can_hold: true,
            controls: default_keyboard_keybindings(),

            moving_right: false,
            moving_left: false,
            soft_dropping: false,

            das: Duration::from_millis(85),
            das_start: None,
            das_charged: false,
            arr: Duration::from_millis(0),
            arr_start: None,
            sds: INFINITY,
            gravity: LEVEL_GRAVITIES[0],
            last_drop: Instant::now(),
            fall_timing: Duration::from_millis((1000. / LEVEL_GRAVITIES[0]) as u64),
            on_ground: false,
            on_ground_start: None,
            actions_from_ground: 0,

            score: 0,
            lines: 0,
            level: 1,
            pieces: 0,
            start_time: Instant::now(),

            t_spin: false,
            t_spin_mini: false,
            prev_clear: false,
            combo: 0,
            latest_clear_difficult: false,
            back_to_back: false,
        }
    }

    pub fn reset_game(&mut self) {
        self.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        self.game_over = false;
        self.held_piece = None;
        self.garbage_queue = VecDeque::new();
        self.piece_queue = VecDeque::new();
        self.spawn_new_piece();

        self.moving_right = false;
        self.moving_left = false;
        self.last_drop = Instant::now();

        self.score = 0;
        self.lines = 0;
        self.level = 1;
        self.pieces = 0;
        self.start_time = Instant::now();

        self.latest_clear_difficult = false;
        self.back_to_back = false;
    }

    // Used for leveling and regular gravity increase 
    pub fn set_gravity_hard(&mut self, gravity: f32) {
        self.gravity = gravity;
        self.fall_timing = Duration::from_millis((1000. / gravity) as u64);
    }
    // Used for soft drop, handles INFINITY edge case
    pub fn set_gravity_soft(&mut self, gravity: f32) {
        self.fall_timing = Duration::from_millis((1000. / gravity) as u64);
    }

    pub fn spawn_new_piece(&mut self) {
        if self.piece_queue.len() < 7 {
            // 7-bag
            let l = PieceType::get_random_as_list();
            for p in l {
                self.piece_queue.push_back(Piece::new(p, 0));
            }
        }

        println!("Spawning new piece...");
        self.active_piece = self.piece_queue.pop_front().unwrap();
        self.can_hold = true;
        self.on_ground = false;

        // Check if spawn location is valid
        if !self.is_valid_position(0, 0) {
            println!("Game Over!");
            self.game_over = true;
        }

        if !self.is_valid_position(0, -1) {
            self.on_ground = true;
            self.on_ground_start = Some(Instant::now());
        }
    }


    pub fn update(&mut self, ctx: &mut Context) {
        if self.game_over {
            if ctx.keyboard.is_key_just_pressed(KeyCode::R) {self.reset_game();}
            return;
        }

        //Handle inputs
        self.handle_game_inputs(ctx);

        // Downward movement (soft drop or natural fall)
        while !self.on_ground && self.last_drop.elapsed() >= self.fall_timing {
            self.last_drop += self.fall_timing;
            if self.move_piece(0, -1) && self.soft_dropping {
                self.score += 1;
            }
        }

        // Horizontal movement
        if let Some(das_start) = self.das_start {
            // Check if DAS is charged
            if !self.das_charged && das_start.elapsed() >= self.das {
                self.das_charged = true;
                self.arr_start = Some(das_start + self.das);
            }

            if let Some(mut arr_start) = self.arr_start {
                // Move if ARR allows
                while arr_start.elapsed() >= self.arr {
                    if self.moving_left {
                        if !self.move_piece(-1, 0) {break}
                    }
                    else if self.moving_right {
                        if !self.move_piece(1, 0) {break}
                    }
                    arr_start += self.arr;
                }
            }
        }
        
        // Place piece if it has been stationary for .5 seconds
        if let Some(t) = self.on_ground_start {
            if self.on_ground && t.elapsed() >= Duration::from_millis(500) {
                self.place_piece();
            }
        }
    }
}
