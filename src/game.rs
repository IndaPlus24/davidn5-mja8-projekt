use std::collections::{HashMap, VecDeque};


use ggez::Context;

use crate::board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS};
use crate::{default_keyboard_keybindings, GameAction, KeyCode, Piece, PieceType};
use crate::consts::{TICKS_BEFORE_NEXT_PIECE, LEVELS_TICK_COUNTS};

pub struct Game {
    pub board: [[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS], 
    pub tick_count: u32,
    pub current_level: usize,
    pub active_piece: Piece,
    pub held_piece: Option<Piece>,
    pub piece_queue: VecDeque<Piece>,
    pub ticks_since_last_input: usize,
    pub ticks_since_last_rotation: usize, 
    pub ticks_without_moving_down: usize,
    pub can_hold: bool,
    pub controls: HashMap<GameAction, KeyCode>,
}

impl Game {
    pub fn new () -> Self {
        let mut piece_queue: VecDeque<Piece> = VecDeque::new();
        let l = PieceType::get_random_as_list();
        for p in l {
            piece_queue.push_back(Piece::new(p, 0));
        }

        let active_piece = piece_queue.pop_front().unwrap();

        Game {
            board: [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
            tick_count: 0,
            current_level: 0,
            active_piece: active_piece, 
            held_piece: None, 
            piece_queue: piece_queue, 
            ticks_since_last_input: 0,
            ticks_since_last_rotation: 0,
            ticks_without_moving_down: 0,
            can_hold: true,
            controls: default_keyboard_keybindings(),
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
        self.ticks_without_moving_down = 0;
        self.can_hold = true;
    }

    pub fn next_tick(&mut self, ctx: &mut Context) {
        self.tick_count += 1;
        self.ticks_since_last_input += 1;
        self.ticks_since_last_rotation +=1;

        //Spawn new Piece
        if self.ticks_without_moving_down == TICKS_BEFORE_NEXT_PIECE {
            self.spawn_new_piece();
        }

        //Handle inputs
        self.handle_inputs(ctx);

        // IF THE TICK COUNT MATCHES THE CURRENT LEVELS TICK COUNT
        if self.tick_count % LEVELS_TICK_COUNTS[self.current_level] == 0 {
            //MOVE PIECE DOWN
            if !self.move_piece(0, -1) {
                self.ticks_without_moving_down += 1;
                self.place_piece();
                println!("Piece at bottom...");
                println!("Checking Lines...");
                self.check_full_line();
                self.spawn_new_piece();
            }
        }
    }

}