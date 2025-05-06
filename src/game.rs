use std::collections::{HashMap, VecDeque};
use std::f32::INFINITY;
use std::time::{Duration, Instant};

use ggez::Context;

use crate::board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS};
use crate::consts::{GameMode, DEFAULT_GRAVITY};
use crate::{default_keyboard_keybindings, GameAction, KeyCode, Piece, PieceType};

#[derive(Clone)]
pub struct Game {
    // Canvas info
    pub canvas_pos: (f32, f32),
    pub canvas_scale: f32,

    // General
    pub board: [[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    pub gamemode: GameMode,

    pub game_over: bool,
    pub objective_completed: bool,
    pub garbage_queue: VecDeque<(usize, usize)>, // (amount, column of garbage hole)
    pub active_piece: Piece,
    pub held_piece: Option<PieceType>,
    pub piece_queue: VecDeque<PieceType>,
    pub can_hold: bool,
    pub controls: HashMap<GameAction, KeyCode>,
    pub continue_to_highscore : bool,

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
    pub lowest_row: isize, // Lowest row that piece has touched
    pub action_count: usize, // Action counter. If it reaches 15, the piece will automatically lock in place

    // Stats
    pub score: usize,
    pub lines: usize,
    pub level: usize,
    pub pieces: usize,
    pub attack: usize,
    pub start_time: Instant,
    pub final_time: Duration,

    // Scoring checks
    pub t_spin: bool,
    pub t_spin_mini: bool,
    pub prev_clear: bool, // true if previous piece resulted in a line clear
    pub combo: usize,
    pub latest_clear_difficult: bool, // true if latest line clear was a tetris or t-spin
    pub back_to_back: bool,
    pub all_clear: bool,
}

impl Game {
    pub fn new(pos: (f32, f32), scl: f32) -> Self {
        Game {
            canvas_pos: pos,
            canvas_scale: scl,

            board: [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
            gamemode: GameMode::FourtyLines,

            game_over: false,
            objective_completed: false,
            garbage_queue: VecDeque::new(),
            active_piece: Piece::new(PieceType::Z, 0),
            held_piece: None,
            piece_queue: VecDeque::new(),
            can_hold: true,
            controls: default_keyboard_keybindings(),
            continue_to_highscore : false,

            moving_right: false,
            moving_left: false,
            soft_dropping: false,

            das: Duration::from_millis(85),
            das_start: None,
            das_charged: false,
            arr: Duration::from_millis(0),
            arr_start: None,
            sds: INFINITY,
            gravity: DEFAULT_GRAVITY,
            last_drop: Instant::now(),
            fall_timing: Duration::from_millis((1000. / DEFAULT_GRAVITY) as u64),
            on_ground: false,
            on_ground_start: None,
            lowest_row: 21,
            action_count: 0,

            score: 0,
            lines: 0,
            level: 1,
            pieces: 0,
            attack: 0,
            start_time: Instant::now(),
            final_time: Duration::from_secs(0),

            t_spin: false,
            t_spin_mini: false,
            prev_clear: false,
            combo: 0,
            latest_clear_difficult: false,
            back_to_back: false,
            all_clear: false,
        }
    }

    pub fn reset_game(&mut self) {
        self.board = [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS];
        self.game_over = false;
        self.held_piece = None;
        self.garbage_queue = VecDeque::new();
        self.piece_queue = VecDeque::new();
        self.spawn_piece_from_queue();
        self.continue_to_highscore  = false;

        self.moving_right = false;
        self.moving_left = false;
        self.last_drop = Instant::now();

        self.score = 0;
        self.lines = 0;
        self.set_level(1);
        self.pieces = 0;
        self.attack = 0;
        self.start_time = Instant::now();

        self.latest_clear_difficult = false;
        self.back_to_back = false;
    }

    pub fn end_game(&mut self, objective_completed: bool) {
        self.game_over = true;
        self.final_time = self.start_time.elapsed();
        self.objective_completed = objective_completed;
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


    pub fn spawn_piece(&mut self, piece_type: PieceType) {
        self.active_piece = Piece::new(piece_type, 0);
        self.last_drop = Instant::now();

        // Check if spawn location is valid
        if !self.is_valid_position(0, 0) {
            self.end_game(false);
        }

        // On ground check
        self.on_ground = false;
        if !self.is_valid_position(0, -1) {
            self.on_ground = true;
            self.on_ground_start = Some(Instant::now());
        }
    }

    pub fn spawn_piece_from_queue(&mut self) {
        // Generate new bag if piece queue is shorter than 7 pieces
        if self.piece_queue.len() < 7 {
            let mut l = PieceType::get_random_as_list();
            self.piece_queue.append(&mut l);
        }

        let next_piece_type = self.piece_queue.pop_front().unwrap();
        self.spawn_piece(next_piece_type);
        self.can_hold = true;
    }


    pub fn update(&mut self, ctx: &mut Context) {
        if self.game_over {
            if ctx.keyboard.is_key_just_pressed(KeyCode::R) {self.reset_game();}
            if ctx.keyboard.is_key_just_pressed(*self.controls.get(&GameAction::HardDrop).unwrap()){
                self.continue_to_highscore = true;
            }
            return;
        }

        // Downward movement (soft drop or natural fall)
        while !self.on_ground && self.last_drop.elapsed() >= self.fall_timing {
            self.last_drop += self.fall_timing;
            if self.move_piece(0, -1) {
                if self.soft_dropping {
                    self.score += 1;
                }
                if self.is_new_lowest() {
                    self.action_count = 0;
                }
            }

            self.on_ground_check();
        }

        //Handle inputs
        self.handle_game_inputs(ctx);

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
                        else {self.add_action()}
                    }
                    else if self.moving_right {
                        if !self.move_piece(1, 0) {break}
                        else {self.add_action()}
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

    pub fn on_ground_check(&mut self) {
        if !self.is_valid_position(0, -1) {
            self.on_ground = true;
            self.on_ground_start = Some(Instant::now());
        } else {
            self.on_ground = false;
            self.on_ground_start = None;
        }
    }

    pub fn add_action(&mut self) {
        self.action_count += 1;

        self.on_ground_check();
        // Check if piece has reached a new lowest row
        if self.is_new_lowest() {
            if !self.on_ground {
                self.action_count = 0;
            }
        }

        if self.action_count >= 15 && self.on_ground {
            self.place_piece();
        }
    }

    pub fn is_new_lowest(&mut self) -> bool {
        let prev_lowest = self.lowest_row;
        for (dr, _) in &self.active_piece.block_positions {
            let r = self.active_piece.midpoint.0 + dr;
            if r < self.lowest_row {
                self.lowest_row = r;
            }
        }

        prev_lowest != self.lowest_row
    }
}