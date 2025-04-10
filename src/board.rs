use std::collections::HashSet;

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

    pub fn can_move_direction(&mut self, piece: &mut Piece, dx: isize, dy: isize) -> bool {
        piece.block_positions.iter().all(|&(r, c)| {
            let new_c = c as isize + dx;
            let new_r = r as isize + dy;

            if new_c < 0
                || new_c >= BOARD_AMOUNT_COLUMNS as isize
                || new_r < 0
                || new_r >= BOARD_AMOUNT_ROWS as isize
            {
                return false;
            }

            let new_pos = (new_r as usize, new_c as usize);

            !self.table[new_pos.0][new_pos.1].is_occupied()
                || piece.block_positions.contains(&new_pos)
        })
    }

    pub fn move_piece(&mut self, piece: &mut Piece, dx: isize, dy: isize) -> bool {
        if !self.can_move_direction(piece, dx, dy) {
            return false;
        }

        // CLEAR CURRENT POS
        for &(r, c) in &piece.block_positions {
            self.table[r][c].path = "empty".to_string();
            self.table[r][c].occupied = false;
        }

        // MOVE PIECES
        for (r, c) in &mut piece.block_positions {
            *c = (*c as isize + dx) as usize;
            *r = (*r as isize + dy) as usize;

            self.table[*r][*c].path = piece.piece_type.get_path();
            self.table[*r][*c].occupied = true;
        }

        true
    }

    pub fn hard_drop(&mut self, piece: &mut Piece) -> bool {
        while self.move_piece(piece, 0, 1) {}
        true
    }

    pub fn check_full_line(&mut self, piece: &Piece) {
        let rows_to_check: HashSet<usize> = piece.block_positions.iter().map(|&(r, _)| r).collect();

        let mut rows_to_remove: Vec<usize> = Vec::new();
        for row in rows_to_check {
            if self.table[row].iter().all(|b| b.is_occupied()) {
                println!("ROW: {} IS FULL", row);
                rows_to_remove.push(row);
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
