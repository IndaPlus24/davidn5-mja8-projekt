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

        for r in 0..19 {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                if board[19 - r][c].is_some() && !found_height.contains(&c) {
                    height += (20 - r) as f32;
                    found_height.insert(c);
                }
            }
        }

        height
    }

    pub fn count_holes(
        board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    ) -> f32 {
        let mut holes: f32 = 0.;

        for c in 0..BOARD_AMOUNT_COLUMNS {
            let mut found_block = false;
            for r in (0..BOARD_AMOUNT_ROWS).rev() {
                if board[r][c].is_some() {
                    found_block = true;
                } else if found_block {
                    holes += 1.0;
                }
            }
        }

        holes
    }

    pub fn count_bumpiness(
        board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    ) -> f32 {
        let mut heights = [0; 10];
        let mut found_height: HashSet<usize> = HashSet::new();

        for r in 0..19 {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                if board[19 - r][c].is_some() && !found_height.contains(&c) {
                    heights[c] = 20 - r;
                    found_height.insert(c);
                }
            }
        }

        let mut bumpiness: f32 = 0.;

        for i in 1..heights.len() as usize {
            bumpiness += heights[i - 1].abs_diff(heights[i]) as f32;
        }

        bumpiness
    }

    pub fn count_lines_cleared(
        board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS],
    ) -> f32 {
        let mut lines: usize = 0;
        for row in 0..BOARD_AMOUNT_ROWS {
            if board[row].iter().all(|b| b.is_some()) {
                lines += 1;
            }
        }
        lines as f32
    }

    pub fn simulate_place_piece(&mut self) -> bool {
        let piece = &self.active_piece;
        let (mr, mc) = piece.midpoint;
        piece.block_positions.iter().for_each(|(dr, dc)| {
            self.board[(mr + dr) as usize][(mc + dc) as usize] = Some(piece.piece_type);
        });

        true
    }
}
