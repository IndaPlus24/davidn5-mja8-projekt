use std::time::Instant;

use crate::Game;
use crate::{Piece, PieceType};
use crate::{ROTATION_CW, ROTATION_CCW};
pub use crate::consts::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS};

use crate::rotation::{
    KICK_TABLE_180,
    KICK_TABLE_CCW_I,
    KICK_TABLE_CCW_REGULAR,
    KICK_TABLE_CW_I,
    KICK_TABLE_CW_REGULAR,
};

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
            {
                return false;
            }

            self.board[r as usize][c as usize].is_none()
        })
    }

    pub fn move_piece(&mut self, dx: isize, dy: isize) -> bool {
        if !self.is_valid_position(dx, dy) {
            return false;
        }
        self.active_piece.midpoint.0 += dy;
        self.active_piece.midpoint.1 += dx;
        
        self.on_ground = !self.is_valid_position(0, -1);
        self.on_ground_start = Some(Instant::now());
        
        true
    }

    pub fn place_piece(&mut self) -> bool {
        let piece = &self.active_piece;
        let (mr, mc) = piece.midpoint;
        piece.block_positions.iter().for_each(|(dr, dc)| {
            self.board[(mr+dr) as usize][(mc+dc) as usize] = Some(piece.piece_type);
        });
        // TODO: calculate score
        self.check_full_line();
        self.spawn_new_piece();
        true
    }

    pub fn hard_drop(&mut self) -> bool {
        while self.move_piece(0, -1) {}
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
    
    pub fn rotate(&mut self, rotation_type: usize) -> bool {
        let piece = self.active_piece.clone();
        let new_rotation: usize = (piece.rotation + rotation_type) % 4;

        // Set up rotated piece for kick table checks
        let mut rotated_piece = Piece::new(piece.piece_type, new_rotation);
        rotated_piece.midpoint = piece.midpoint;
        self.active_piece = rotated_piece;

        // Fetch the suitable kick table
        let kick_table = match rotation_type {
            ROTATION_CW => match piece.piece_type {
                PieceType::I => KICK_TABLE_CW_I[new_rotation],
                _ => KICK_TABLE_CW_REGULAR[new_rotation],
            },
            ROTATION_CCW => match piece.piece_type {
                PieceType::I => KICK_TABLE_CCW_I[new_rotation],
                _ => KICK_TABLE_CCW_REGULAR[new_rotation],
            },
            _ => KICK_TABLE_180[new_rotation],
        };
        
        // Try kick table offsets
        for (dx, dy) in kick_table {
            if self.move_piece(dx, dy) {
                self.active_piece.rotation = new_rotation;
                return true;
            }
        }

        self.active_piece = piece;
        false
    }

    pub fn check_full_line(&mut self) {

        let mut rows_to_remove: Vec<usize> = Vec::new();
        for row in 0..BOARD_AMOUNT_ROWS {
            // CHECK IF ROW IS FULL
            if self.board[row as usize].iter().all(|b| {
                match b {
                    Some(_) => true,
                    None => false
                }
            }) { // cursed lol
                println!("ROW: {} IS FULL", row);
                rows_to_remove.push(row as usize);
            }
        }
        if !rows_to_remove.is_empty() {
            rows_to_remove.reverse();

            //MOVE DOWN THE ROWS ABOVE
            for &row in &rows_to_remove {
                for r in row..BOARD_AMOUNT_ROWS-1 {
                    self.board[r] = self.board[r + 1].clone()
                }
            }
        }
    }
}
