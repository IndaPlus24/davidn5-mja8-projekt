use ggez::graphics::Color;

pub const BLOCK_SIZE: i32 = 25;
pub const EMPTY_BLOCK_COLOR: Color = Color {
    g: 1.,
    b: 1.,
    r: 1.,
    a: 255.,
};

// All of these consts should probably be relative to window size 
pub const BOARD_AMOUNT_COLUMNS: usize = 10;
pub const BOARD_AMOUNT_ROWS: usize = 40;
pub const BOARD_LOWER_LEFT: (i32, i32) = (200, 520);
pub const HOLD_PIECE_UPPERLEFT: (isize,isize) = (BOARD_LOWER_LEFT.0 as isize - 140, BOARD_LOWER_LEFT.1 as isize - 50);
pub const HOLD_PIECE_MIDDLE : (isize,isize) = (HOLD_PIECE_UPPERLEFT.0 + BLOCK_SIZE as isize * 2 - 20 ,HOLD_PIECE_UPPERLEFT.1 + BLOCK_SIZE as isize * 2);

pub const LEVELS_TICK_COUNTS: [u32; 1] = [60];
pub const TICKS_BETWEEN_INPUTS: usize = 2;
pub const TICKS_BETWEEN_ROTATIONS : usize = 2;
pub const TICKS_BEFORE_NEXT_PIECE: usize = 2;