// All of these consts should probably be relative to window size
pub const WINDOW_WIDTH: f32 = 1920.;
pub const WINDOW_HEIGHT: f32 = 1080.;

pub const GAME_1_POS: (f32, f32) = (100., 100.);
pub const GAME_1_SCL: f32 = 1.;

pub const BOARD_AMOUNT_COLUMNS: usize = 10;
pub const BOARD_AMOUNT_ROWS: usize = 40;

//Delays (seconds)
pub const LEVEL_GRAVITIES: [f32; 1] = [0.8]; // Cells per second

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

pub enum BoardRenderType {
    Marathon,
    FourtyLines,
    Versus,
}
