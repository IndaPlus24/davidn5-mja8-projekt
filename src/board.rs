use crate::{block::Block, BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS, EMPTY_BLOCK_COLOR};
use crate::Piece;


pub struct Board{
    pub table : [[Block; BOARD_AMOUNT_ROWS]; BOARD_AMOUNT_COLUMNS],
}

impl Board {
    pub fn new () -> Self {
        Self {table : [[Block::new(EMPTY_BLOCK_COLOR); BOARD_AMOUNT_ROWS]; BOARD_AMOUNT_COLUMNS]}
    }

    pub fn can_move_direction(&mut self, piece: &mut Piece, dx : isize, dy : isize) -> bool {
        piece.block_positions.iter().all(|&(c, r)| {
            let new_c = c as isize + dx;
            let new_r = r as isize + dy;
    
                if new_c < 0 || new_c >= BOARD_AMOUNT_COLUMNS as isize || new_r < 0 || new_r >= BOARD_AMOUNT_ROWS as isize {
                return false;
            }
    
            let new_pos = (new_c as usize, new_r as usize);
    
            !self.table[new_pos.0][new_pos.1].is_occupied() || piece.block_positions.contains(&new_pos)
        })
    }

    pub fn move_piece(&mut self, piece: &mut Piece, dx: isize, dy: isize) -> bool {
        if !self.can_move_direction(piece, dx, dy) {
            return false;
        }
    
        // Clear current positions
        for &(c, r) in &piece.block_positions {
            self.table[c][r].color = EMPTY_BLOCK_COLOR;
            self.table[c][r].occupied = false;
        }
    
        // Move and redraw
        for (c, r) in &mut piece.block_positions {
            *c = (*c as isize + dx) as usize;
            *r = (*r as isize + dy) as usize;
    
            self.table[*c][*r].color = piece.piece_type.color();
            self.table[*c][*r].occupied = true;
        }
    
        true
    }
    
}