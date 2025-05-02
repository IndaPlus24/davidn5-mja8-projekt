use std::collections::HashSet;

use crate::{
    board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS},
    Game, PieceType,
};

impl Game {
    pub fn get_aggregate_height(
        board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    ) -> f32 {
        let mut height: f32 = 0.;

        let mut found_height: HashSet<usize> = HashSet::new();

        for r in 0..BOARD_AMOUNT_ROWS {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                if board[r][c].is_some() && !found_height.contains(&c) {
                    height += r as f32;
                    found_height.insert(c);
                }
            }
        }

        height / 200.
    }

    pub fn count_holes(
        board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    ) -> f32 {
        let mut holes: f32 = 0.;

        for r in 0..(BOARD_AMOUNT_ROWS as i32 - 1) as usize {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                if board[r][c].is_some() && board[r + 1][c].is_none() {
                    holes += 1.;
                }
            }
        }

        holes / 180.
    }

    pub fn count_bumpiness(
        board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    ) -> f32 {
        let mut heights = [0; 10];
        let mut found_height: HashSet<usize> = HashSet::new();

        for r in 0..BOARD_AMOUNT_ROWS {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                if board[r][c].is_some() && !found_height.contains(&c) {
                    heights[c] = r;
                    found_height.insert(c);
                }
            }
        }

        let mut bumpiness: f32 = 0.;

        for i in 1..heights.len() as usize {
            bumpiness += heights[i - 1].abs_diff(heights[i]) as f32;
        }
        bumpiness / 180.
    }

    pub fn count_lines_cleared(
        board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    ) -> f32 {
        let mut lines: usize = 0;
        for row in 0..BOARD_AMOUNT_ROWS {
            // CHECK IF ROW IS FULL
            if board[row as usize].iter().all(|b| match b {
                Some(_) => true,
                None => false,
            }) {
                lines += 1;
            }
        }
        lines as f32 / 4.
    }

    pub fn simulate_place_piece(&mut self) -> bool {
        let piece = &self.active_piece;
        let (mr, mc) = piece.midpoint;
        piece.block_positions.iter().for_each(|(dr, dc)| {
            self.board[(mr+dr) as usize][(mc+dc) as usize] = Some(piece.piece_type);
        });
        
        true
    }
}