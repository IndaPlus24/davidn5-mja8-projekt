// All of these consts should probably be relative to window size
pub const WINDOW_WIDTH: f32 = 1920.;
pub const WINDOW_HEIGHT: f32 = 1080.;

pub const GAME_1_POS: (f32, f32) = (100., 100.);
pub const GAME_1_SCL: f32 = 1.;

pub const BOARD_AMOUNT_COLUMNS: usize = 10;
pub const BOARD_AMOUNT_ROWS: usize = 40;

//Delays (seconds)
pub const LEVELS_GRAVITY_THRESHOLD: [f32; 1] = [0.8];
pub const TICKS_BETWEEN_INPUTS: f32 = 0.05; // 50 ms between soft drop inputs
pub const TICKS_BETWEEN_ROTATIONS: f32 = 0.1; // 100 ms between rotation inputs
pub const TICKS_BEFORE_NEXT_PIECE: f32 = 0.00; // can probably remove?

pub const DAS_TICKS: f32 = 0.06; // Delayed Auto Shift - ticks between the inital input until execution
pub const ARR_TICKS: f32 = 0.04; // Auto Repeat Rate - ticks between piece movement when holding down

#[derive(PartialEq)]
pub enum GameState{
    StartScreen, 
    MainMenu,
    GameModeSelector,
    Singleplayer,
    Multiplayer, 
    VsBots,  
    GameOver, 
    HighscoreInput
}