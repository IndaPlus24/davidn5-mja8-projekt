
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

pub struct Board {
    pub table: [[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
}

impl Board {
    pub fn new() -> Self {
        Self { table: [[None; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS] }
    }

    pub fn is_valid_position(&mut self, piece: &mut Piece, dx: isize, dy: isize) -> bool {
        let (mr, mc) = piece.midpoint;
        piece.block_positions.iter().all(|(dr, dc)| {
            let r = mr + dr + dy;
            let c = mc + dc + dx;

            if c < 0
                || c >= BOARD_AMOUNT_COLUMNS as isize
                || r >= BOARD_AMOUNT_ROWS as isize
            {
                return false;
            }
            if r < 0 {return true;}

            match self.table[r as usize][c as usize] {
                Some(_) => false,
                None => true
            }
        })
    }

    pub fn move_piece(&mut self, piece: &mut Piece, dx: isize, dy: isize) -> bool {
        if !self.is_valid_position(piece, dx, dy) {
            return false;
        }
        piece.midpoint.0 += dy;
        piece.midpoint.1 += dx;
        true
    }

    pub fn place_piece(&mut self, piece: &mut Piece) -> bool {
        let (mr, mc) = piece.midpoint;
        piece.block_positions.iter().for_each(|(dr, dc)| {
            self.table[(mr+dr) as usize][(mc+dc) as usize] = Some(piece.piece_type);
        });
        true
    }

    pub fn hard_drop(&mut self, piece: &mut Piece) -> bool {
        while self.move_piece(piece, 0, 1) {}
        self.place_piece(piece)
    }

    pub fn get_ghost_piece(&mut self, piece: &Piece) -> Piece {
        let mut ghost = piece.clone();
    
        while self.is_valid_position(&mut ghost, 0, 1) {
            ghost.midpoint.0 += 1;
        }
        ghost.piece_type = PieceType::X;
        ghost
    }
    
    pub fn rotate(&mut self, piece: &mut Piece, rotation_type: usize) -> bool {
        let new_rotation: usize = (piece.rotation + rotation_type) % 4;

        // Set up rotated piece for kick table checks
        let mut rotated_piece = Piece::new(piece.piece_type, new_rotation);
        rotated_piece.midpoint = piece.midpoint;

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
            if self.move_piece(&mut rotated_piece, dx, dy) {
                piece.block_positions = rotated_piece.block_positions;
                piece.rotation = new_rotation;
                piece.midpoint = rotated_piece.midpoint;
                return true;
            }
        }

        false
    }

    pub fn check_full_line(&mut self) {

        let mut rows_to_remove: Vec<usize> = Vec::new();
        for row in 0..BOARD_AMOUNT_ROWS {
            if self.table[row as usize].iter().all(|b| {
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
            // SORT ROWS IN ASCENDING ORDER SO WE START CLEARING FROM BOTTOM
            rows_to_remove.sort();

            //MOVE DOWN THE ROWS ABOVE
            for &row in &rows_to_remove {
                for r in (1..=row).rev() {
                    self.table[r] = self.table[r - 1].clone()
                }
            }

            //CLEAR TOP ROW
            for i in 0..BOARD_AMOUNT_COLUMNS {
                self.table[0][i] = None;
            }
        }
    }
}
