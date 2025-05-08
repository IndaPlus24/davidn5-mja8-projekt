use std::time::Instant;

//Holds various information on the various animations
pub struct AnimationState {
    pub start_screen_y: f32,
    pub velocity: f32,
    pub selected_item_main_menu: i32,
    pub selected_item_gamemode_selector: i32,
    pub selected_item_singleplayer_selector: i32,
    pub selected_item_marathon_prompt: (usize, usize),
    pub selected_item_reset_selector: i32,
    pub selected_item_bot_selector: i32,

    pub players_ready: (bool, bool),
    pub selected_item_high_score: (i32,i32),
    pub highscore_list: Vec<(String, usize)>,

    pub selected_key: (usize, usize),
    pub ticks: usize,
    pub size_index: usize, 
    pub name_input: String,
    pub name_ready: bool,

    pub selected_item_settings: usize,
    pub last_setting_tick: Option<Instant>,
}

impl AnimationState {
    pub fn new(highscore_list: Vec<(String, usize)>) -> Self {
        Self {
            // Bouncing text in start_screen.rs
            start_screen_y: -50.,
            velocity: 0.,

            // Arrow Main Menu
            selected_item_main_menu: 0, // (0 - START GAME)....(1 - SETTINGS)

            // Arrow GameMode Selector
            selected_item_gamemode_selector: 0, // (0 - 1v1)....(1 - singleplayer) .....(2 - vs bots)

            // Arrow Singleplayer Selector
            selected_item_singleplayer_selector: 0, // (0 - Marathon)....(1 - 40L) .....(2 - Survival)

            // Arrow(s) Marathon Prompt
            selected_item_marathon_prompt: (1, 0), // ((n, 0) - Start Level) ... ((n, 1) - Confirm)

            // Arrow Reset Selector
            selected_item_reset_selector: 0, // (0 - Reset) ... (1 - Main Menu)

            // Arrow Bot Selector
            selected_item_bot_selector: 0,

            // Ready checks for player one and two. Used in both rematch prompt and initial ready check
            players_ready: (false, false),

            // Arrow High Score
            selected_item_high_score: (0, 0),
            highscore_list,

            //Input Name
            selected_key: (0,0),
            ticks: 0,
            size_index: 0,
            name_input: String::from(""),
            name_ready: false,

            //Settings 
            selected_item_settings: 0,
            last_setting_tick: None,
        }
    }
}
