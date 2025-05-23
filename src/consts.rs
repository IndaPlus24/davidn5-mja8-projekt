// All of these consts should probably be relative to window size
pub const WINDOW_WIDTH: f32 = 1920.;
pub const WINDOW_HEIGHT: f32 = 1080.;

pub const GAME_1_SOLO_POS: (f32, f32) = (550., 100.);
pub const GAME_1_SOLO_SCL: f32 = 1.25;

pub const GAME_1_VS_POS: (f32, f32) = (200., 200.);
pub const GAME_1_VS_SCL: f32 = 1.;

pub const GAME_2_VS_POS: (f32, f32) = (1028., 200.);
pub const GAME_2_VS_SCL: f32 = 1.;

pub const SETTINGS_TICK_SPEED: u128 = 150; // in millis

pub const DEFAULT_DAS: u64 = 165; // in millis
pub const DEFAULT_ARR: u64 = 35; // in millis
pub const DEFAULT_SDS: f32 = 15.; // in cells per seconds

pub const GARBAGE_CAP: usize = 8;
pub const GARBAGE_DELAY: u128 = 1000; // time in milliseconds before garbage can appear on board

pub const SURVIVAL_TIMER: u64 = 1000; // time in millis between each line spawn

pub const BOARD_AMOUNT_COLUMNS: usize = 10;
pub const BOARD_AMOUNT_ROWS: usize = 40;

//Delays (seconds)
pub const DEFAULT_GRAVITY: f32 = 1.; // Cells per second
pub const BOT_DIFFICULTY_SPEEDS: [f32 ; 3] = [0.5, 3., 5.];

#[derive(PartialEq)]
pub enum ScreenState {
    StartScreen,
    MainMenu,
    GameModeSelector,
    SingleplayerSelector,
    Singleplayer,
    MarathonPrompt,
    FourtyLinesReset,
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
