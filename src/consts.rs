// All of these consts should probably be relative to window size
pub const WINDOW_WIDTH: f32 = 1920.;
pub const WINDOW_HEIGHT: f32 = 1080.;

pub const GAME_1_POS: (f32, f32) = (200., 100.);
pub const GAME_1_SCL: f32 = 1.;

pub const GAME_2_POS: (f32, f32) = (1028., 100.);
pub const GAME_2_SCL: f32 = 1.;

pub const GARBAGE_CAP: usize = 8;
pub const GARBAGE_DELAY: u128 = 2000; // time in milliseconds before garbage can appear on board

pub const BOARD_AMOUNT_COLUMNS: usize = 10;
pub const BOARD_AMOUNT_ROWS: usize = 40;

//Delays (seconds)
pub const DEFAULT_GRAVITY: f32 = 1.; // Cells per second
pub const BOT_DIFFICULTY_SPEEDS : [f32 ; 3] = [5., 10. ,20.];

#[derive(PartialEq)]
pub enum ScreenState {
    StartScreen,
    MainMenu,
    GameModeSelector,
    SingleplayerSelector,
    Singleplayer,
    VersusReady,
    Versus,
    VersusRematch,
    VsBots,
    HighscoreInput,
    Settings,
    BotSelector,
    HighScore,
}

#[derive(Clone, PartialEq)]
pub enum GameMode {
    Marathon,
    FourtyLines,
    Survival,
    Versus,
}
