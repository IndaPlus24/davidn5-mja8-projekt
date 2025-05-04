use crate::{
    board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS},
    Game, PieceType,
};

impl Game {
    pub fn get_aggregate_height(board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS]) -> f32 {
        (0..BOARD_AMOUNT_COLUMNS)
            .map(|c| column_height(board, c) as f32)
            .sum()
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

    pub fn count_bumpiness(board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS]) -> f32 {
        let heights: Vec<usize> = (0..BOARD_AMOUNT_COLUMNS)
            .map(|c| column_height(board, c))
            .collect();
    
        heights
            .windows(2)
            .map(|w| (w[0] as i32 - w[1] as i32).abs() as f32)
            .sum()
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

    pub fn compute_well_depth(board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS]) -> i32 {
        let mut total_depth = 0;
        for col in 0..BOARD_AMOUNT_COLUMNS {
            let mut depth = 0;
            for row in 0..BOARD_AMOUNT_ROWS {
                let cell = board[row][col];
                let left = if col == 0 { Some(PieceType::I) } else { board[row][col - 1] };
                let right = if col == BOARD_AMOUNT_COLUMNS - 1 { Some(PieceType::I) } else { board[row][col + 1] };
    
                if cell.is_none() && left.is_some() && right.is_some() {
                    depth += 1;
                    total_depth += depth;
                } else {
                    depth = 0;
                }
            }
        }
        total_depth
    }
    
}

pub fn column_height(board: &[[Option<PieceType>; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS], col: usize) -> usize {
    for row in (0..BOARD_AMOUNT_ROWS).rev() {
        if board[row][col].is_some() {
            return row + 1;
        }
    }
    0
}