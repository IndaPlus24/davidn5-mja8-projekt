use std::collections::HashSet;

use crate::rotation::KICK_TABLE_CW_REGULAR;
use crate::{block, piece, Piece};
use crate::{block::Block, BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS, EMPTY_BLOCK_COLOR};

pub struct Board {
    pub table: [[Block; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
}

impl Board {
    pub fn new() -> Self {
        let mut table: [[Block; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS] = Default::default();

        for row in table.iter_mut() {
            for item in row.iter_mut() {
                *item = Block::new();
            }
        }

        Self { table }
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

            !self.table[r as usize][c as usize].is_occupied()
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
            self.table[(mr+dr) as usize][(mc+dc) as usize].occupied = true;
            self.table[(mr+dr) as usize][(mc+dc) as usize].path = piece.piece_type.get_path();
        });
        true
    }

    pub fn hard_drop(&mut self, piece: &mut Piece) -> bool {
        while self.move_piece(piece, 0, 1) {}
        self.place_piece(piece)
    }

    pub fn rotate_cw(&mut self, piece: &mut Piece) -> bool {
        let new_rotation: usize = (piece.rotation + 1) % 4;
        let mut rotated_piece = Piece::new(piece.piece_type, new_rotation);
        rotated_piece.midpoint = piece.midpoint;
        
        for (dx, dy) in KICK_TABLE_CW_REGULAR[new_rotation] {
            if self.move_piece(&mut rotated_piece, dx, dy) {
                println!("({}, {}) success!", dx, dy);
                piece.block_positions = rotated_piece.block_positions;
                piece.rotation = new_rotation;
                piece.midpoint = rotated_piece.midpoint;
                return true;
            }
        }

        false
    }

    pub fn rotate_ccw(&mut self, piece: &mut Piece) -> bool {
        let new_rotation: usize = (piece.rotation + 3) % 4;
        let rotated_piece = Piece::new(piece.piece_type, new_rotation);
        
        piece.block_positions = rotated_piece.block_positions;
        piece.rotation = new_rotation;

        true
    }

    pub fn check_full_line(&mut self, piece: &Piece) {
        let rows_to_check: HashSet<isize> = piece.block_positions.iter().map(|&(r, _)| r).collect();

        let mut rows_to_remove: Vec<usize> = Vec::new();
        for row in rows_to_check {
            if self.table[row as usize].iter().all(|b| b.is_occupied()) {
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
                self.table[0][i] = Block::new();
            }
        }
    }
}
