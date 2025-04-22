use ggez::graphics::Color;

pub const BLOCK_SIZE: i32 = 25;
pub const EMPTY_BLOCK_COLOR: Color = Color {
    g: 1.,
    b: 1.,
    r: 1.,
    a: 255.,
};

// All of these consts should probably be relative to window size
pub const WINDOW_WIDTH: f32 = 1600.;
pub const WINDOW_HEIGHT: f32 = 900.;

pub const BOARD_AMOUNT_COLUMNS: usize = 10;
pub const BOARD_AMOUNT_ROWS: usize = 40;
pub const BOARD_LOWER_LEFT: (i32, i32) = (200, 600);
pub const HOLD_PIECE_UPPERLEFT: (isize, isize) = (
    BOARD_LOWER_LEFT.0 as isize - 140,
    BOARD_LOWER_LEFT.1 as isize - 50,
);
pub const HOLD_PIECE_MIDDLE: (isize, isize) = (
    HOLD_PIECE_UPPERLEFT.0 + BLOCK_SIZE as isize * 2 - 20,
    HOLD_PIECE_UPPERLEFT.1 + BLOCK_SIZE as isize * 2,
);

//Delays (seconds)
pub const LEVELS_GRAVITY_THRESHOLD: [f32; 1] = [0.8];
pub const TICKS_BETWEEN_INPUTS: f32 = 0.05; // 50 ms between sotft drop inputs
pub const TICKS_BETWEEN_ROTATIONS: f32 = 0.1; // 100 ms between rotation inputs
pub const TICKS_BEFORE_NEXT_PIECE: f32 = 0.00; // can probably remove?

pub const DAS_TICKS: f32 = 0.06; // Delayed Auto Shift - ticks between the inital input until execution
pub const ARR_TICKS: f32 = 0.04; // Auto Repeat Rate - ticks between piece movement when holding down
