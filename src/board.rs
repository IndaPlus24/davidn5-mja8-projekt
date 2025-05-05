use std::time::Instant;
use crate::consts::GameMode;
use crate::gamemodes::versus::get_attack_value;
use crate::scoring::ScoreType;
use crate::Game;
use crate::Piece;
pub use crate::consts::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS};

impl Game {
    pub fn is_valid_position(&mut self, dx: isize, dy: isize) -> bool {
        let piece = &self.active_piece;
        let (mr, mc) = piece.midpoint;
        piece.block_positions.iter().all(|(dr, dc)| {
            let r = mr + dr + dy;
            let c = mc + dc + dx;

            if c >= BOARD_AMOUNT_COLUMNS as isize
                || c < 0
                || r < 0
                || r >= BOARD_AMOUNT_ROWS as isize
            {
                return false;
            }

            self.board[r as usize][c as usize].is_none()
        })
    }

    pub fn is_solid_tile(&mut self, r: isize, c: isize) -> bool {
        if c >= BOARD_AMOUNT_COLUMNS as isize
        || c < 0
        || r < 0
        || r >= BOARD_AMOUNT_ROWS as isize
        {return true;}

        self.board[r as usize][c as usize].is_some()
    }

    pub fn move_piece(&mut self, dx: isize, dy: isize) -> bool {
        if !self.is_valid_position(dx, dy) {
            return false;
        }
        self.active_piece.midpoint.0 += dy;
        self.active_piece.midpoint.1 += dx;

        self.t_spin = false;
        self.t_spin_mini = false;

        true
    }

    pub fn place_piece(&mut self) -> bool {
        let piece = &self.active_piece;
        let (mr, mc) = piece.midpoint;
        piece.block_positions.iter().for_each(|(dr, dc)| {
            self.board[(mr+dr) as usize][(mc+dc) as usize] = Some(piece.piece_type);
        });

        let score_type = self.get_score_type();
        self.add_score(&score_type);

        let attack = get_attack_value(&score_type, self.back_to_back, self.combo);
        // TODO: send 10 extra attack if all clear  
        if attack > 0 {println!("Attack: {}, b2b: {}", attack, self.back_to_back)}
        self.attack += attack;
        if self.all_clear {self.attack += 10}

        self.pieces += 1;

        self.spawn_piece_from_queue();
        self.last_drop = Instant::now();

        self.lowest_row = 21;
        self.action_count = 0;

        true
    }

    pub fn hard_drop(&mut self) -> bool {
        while self.move_piece(0, -1) {
            self.score += 2;
        }
        self.place_piece()
    }

    pub fn get_ghost_piece(&mut self) -> Piece {
        let mut ghost = self.active_piece.clone();
    
        let mut dy: isize = -1;
        while self.is_valid_position(0, dy) {
            dy -= 1;
        }
        ghost.midpoint.0 += dy+1;
        ghost
    }

    pub fn get_score_type(&mut self) -> Option<ScoreType> {
        // Clear lines
        let mut rows_to_remove: Vec<usize> = Vec::new();
        for row in 0..BOARD_AMOUNT_ROWS {
            // CHECK IF ROW IS FULL
            if self.board[row as usize].iter().all(|b| {
                match b {
                    Some(_) => true,
                    None => false
                }
            }) {
                rows_to_remove.push(row as usize);
            }
        }

        let lines_cleared = rows_to_remove.len();

        if !rows_to_remove.is_empty() {
            rows_to_remove.reverse();

            //MOVE DOWN THE ROWS ABOVE
            for &row in &rows_to_remove {
                for r in row..BOARD_AMOUNT_ROWS-1 {
                    self.board[r] = self.board[r + 1].clone()
                }
            }
        }

        self.lines += lines_cleared;

        // Check gamemode specific conditions
        match self.gamemode {
            GameMode::Marathon => {
                if self.lines / 10 == self.level {
                    self.level_up();
                }
            },
            GameMode::FourtyLines => {
                if self.lines >= 40 {
                    self.end_game(true);
                }
            },
            _ => (),
        }


        match lines_cleared {
            0 => {
                if self.t_spin_mini {Some(ScoreType::TSpinMini)}
                else if self.t_spin {Some(ScoreType::TSpin)}
                else {None}
            },
            1 => {
                if self.t_spin_mini {Some(ScoreType::TSpinMiniSingle)}
                else if self.t_spin {Some(ScoreType::TSpinSingle)}
                else {Some(ScoreType::Single)}
            },
            2 => {
                if self.t_spin {Some(ScoreType::TspinDouble)}
                else {Some(ScoreType::Double)}
            },
            3 => {
                if self.t_spin {Some(ScoreType::TSpinTriple)}
                else {Some(ScoreType::Triple)}
            },
            _ => {
                Some(ScoreType::Tetris)
            }
        }
    }
}
