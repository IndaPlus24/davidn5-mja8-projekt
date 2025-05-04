// All of these consts should probably be relative to window size
pub const WINDOW_WIDTH: f32 = 1920.;
pub const WINDOW_HEIGHT: f32 = 1080.;

pub const GAME_1_POS: (f32, f32) = (100., 100.);
pub const GAME_1_SCL: f32 = 1.;

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
    Singleplayer,
    Multiplayer,
    VsBots,
    GameOver,
    HighscoreInput,
    Settings,
    BotSelector,
}

#[derive(Clone, PartialEq)]
pub enum GameMode {
    Marathon,
    FourtyLines,
    Versus,
}
