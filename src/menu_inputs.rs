use std::{f32::INFINITY, time::{Duration, Instant}};

use rand::Rng;

use crate::{animation_state::AnimationState, consts::{GameMode, SETTINGS_TICK_SPEED}, get_scores_from_file, AppState, Game, GameAction, KeyCode, ScreenState};

#[allow(non_snake_case)]
pub struct MenuInputs {
    UP :KeyCode,
    DOWN : KeyCode,
    LEFT : KeyCode,
    RIGHT : KeyCode,
    SELECT : KeyCode
}

impl MenuInputs {
    pub fn pc_inputs() -> Self {
        Self{
            UP: KeyCode::Up,
            DOWN: KeyCode::Down,
            LEFT: KeyCode::Left, 
            RIGHT: KeyCode::Right, 
            SELECT:KeyCode::Space,
        }
    }

    pub fn drifarkaden_inputs() -> Self {
        Self{
            UP: KeyCode::Up,
            DOWN: KeyCode::Down,
            LEFT: KeyCode::Left, 
            RIGHT: KeyCode::Right, 
            SELECT:KeyCode::LAlt,
        }
    }
}

pub fn handle_start_screen_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, menuinputs : &MenuInputs) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(menuinputs.SELECT) {
        *screen_state = ScreenState::MainMenu;
    }
}

pub fn handle_main_menu_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, menuinputs: &MenuInputs) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(menuinputs.DOWN) {
        animation_state.selected_item_main_menu = (animation_state.selected_item_main_menu + 1) % 3;
    } else if keyboard.is_key_just_pressed(menuinputs.UP) {
        animation_state.selected_item_main_menu = (animation_state.selected_item_main_menu + 2) % 3;
    } else if keyboard.is_key_just_pressed(menuinputs.SELECT) {
        *screen_state = match animation_state.selected_item_main_menu {
            0 => {
                animation_state.selected_item_gamemode_selector = 0;
                ScreenState::GameModeSelector
            }
            1 => {
                animation_state.selected_item_high_score = (0, 0);
                ScreenState::HighScore
            }
            _=> {
                animation_state.selected_item_settings = (0, 0);
                ScreenState::Settings
            }
        };
    }
}

pub fn handle_gamemode_selector_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, menuinputs: &MenuInputs) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(menuinputs.DOWN) {
        animation_state.selected_item_gamemode_selector = (animation_state.selected_item_gamemode_selector + 1) % 4;
    } else if keyboard.is_key_just_pressed(menuinputs.UP) {
        animation_state.selected_item_gamemode_selector = (animation_state.selected_item_gamemode_selector + 3) % 4;
    } else if keyboard.is_key_just_pressed(menuinputs.SELECT) {
        *screen_state = match animation_state.selected_item_gamemode_selector {
            0 => ScreenState::VersusReady,
            1 => {
                animation_state.selected_item_singleplayer_selector = 0;
                ScreenState::SingleplayerSelector
            }
            2 => {
                animation_state.selected_item_bot_selector = 0;
                ScreenState::BotSelector
            }
            _ => ScreenState::MainMenu
        }
    }
}

pub fn handle_singleplayer_selector_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, game: &mut Game, menuinputs: &MenuInputs) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(menuinputs.DOWN) {
        animation_state.selected_item_singleplayer_selector = (animation_state.selected_item_singleplayer_selector + 1) % 4;
    } else if keyboard.is_key_just_pressed(menuinputs.UP) {
        animation_state.selected_item_singleplayer_selector = (animation_state.selected_item_singleplayer_selector + 3) % 4;
    } else if keyboard.is_key_just_pressed(menuinputs.SELECT) {
        let selected = animation_state.selected_item_singleplayer_selector;
        *screen_state = match selected {
            0 => {
                animation_state.selected_item_marathon_prompt = (1, 0);
                ScreenState::MarathonPrompt
            }
            3 => ScreenState::GameModeSelector,
            _ => {
                game.reset_game(None);
                match selected {
                    1 => game.gamemode = GameMode::FourtyLines,
                    _ => game.gamemode = GameMode::Survival
                }
                ScreenState::Singleplayer
            }
        }
    }
}

pub fn handle_marathon_prompt_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, game: &mut Game, menuinputs: &MenuInputs) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(menuinputs.DOWN) {
        animation_state.selected_item_marathon_prompt.1 = (animation_state.selected_item_marathon_prompt.1 + 1) % 3;
    }
    if keyboard.is_key_just_pressed(menuinputs.UP) {
        animation_state.selected_item_marathon_prompt.1 = (animation_state.selected_item_marathon_prompt.1 + 2) % 3;
    }
    
    if animation_state.selected_item_marathon_prompt.1 == 0 {
        if keyboard.is_key_just_pressed(menuinputs.LEFT) { // Decrease starting level
            let mut new_level = animation_state.selected_item_marathon_prompt.0 - 1;
            if new_level == 0 {new_level = 1}
            animation_state.selected_item_marathon_prompt.0 = new_level;
        }
        if keyboard.is_key_just_pressed(menuinputs.RIGHT) { // Increase starting level
            let mut new_level = animation_state.selected_item_marathon_prompt.0 + 1;
            if new_level == 16 {new_level = 15}
            animation_state.selected_item_marathon_prompt.0 = new_level;
        }
    }
    if keyboard.is_key_just_pressed(menuinputs.SELECT) {
        if animation_state.selected_item_marathon_prompt.1 == 2 {
            *screen_state = ScreenState::SingleplayerSelector
        } else {
            game.reset_game(None);
            game.set_level(animation_state.selected_item_marathon_prompt.0);
            game.gamemode = GameMode::Marathon;
            *screen_state = ScreenState::Singleplayer
        }    
    }
}

pub fn handle_reset_screen_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, game: &mut Game, menuinputs: &MenuInputs) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(menuinputs.DOWN)
    || keyboard.is_key_just_pressed(menuinputs.UP) {
        animation_state.selected_item_reset_selector = (animation_state.selected_item_reset_selector + 1) % 2;
    } else if keyboard.is_key_just_pressed(menuinputs.SELECT) {
        let selected = animation_state.selected_item_reset_selector;
        *screen_state = match selected {
            0 => {
                game.reset_game(None);
                ScreenState::Singleplayer
            }
            _ => ScreenState::MainMenu
        }
    }
}

pub fn handle_versus_prepost_inputs(
    ctx: &ggez::Context,
    screen_state: &mut ScreenState,
    animation_state: &mut AnimationState,
    game_one: &mut Game,
    game_two: &mut Game
) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(*game_one.controls.get(&GameAction::HardDrop).unwrap()) {
        animation_state.players_ready.0 = !animation_state.players_ready.0;
    }
    if keyboard.is_key_just_pressed(*game_two.controls.get(&GameAction::HardDrop).unwrap()) {
        animation_state.players_ready.1 = !animation_state.players_ready.1;
    }

    if animation_state.players_ready.0 && animation_state.players_ready.1 {
        let mut rng = rand::rng(); 
        let id = Some(rng.random());
        game_one.reset_game(id);
        game_two.reset_game(id);
        *screen_state = ScreenState::Versus;
    }
}

pub fn handle_bot_selector_inputs(ctx: &ggez::Context, state: &mut AppState) {
    let animation_state = &mut state.animation_state;
    let screen_state = &mut state.screen_state;
    let bot = &mut state.bot;
    let menuinputs = &state.menuinputs;

    let mut rng = rand::rng(); 
    let id = Some(rng.random());

    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(menuinputs.DOWN) {
        animation_state.selected_item_bot_selector = (animation_state.selected_item_bot_selector + 1) % 4;
    } else if keyboard.is_key_just_pressed(menuinputs.UP) {
        animation_state.selected_item_bot_selector = (animation_state.selected_item_bot_selector + 3) % 4;
    } else if keyboard.is_key_just_pressed(menuinputs.SELECT) {
        *screen_state = match animation_state.selected_item_bot_selector {
            0 => {
                bot.difficulty = 0;
                bot.game.reset_game(id);
                state.game_one.reset_game(id);
                ScreenState::VsBots
            },
            1 => {
                bot.difficulty = 1;
                bot.game.reset_game(id);
                state.game_one.reset_game(id);
                ScreenState::VsBots
            },
            2 => {
                bot.difficulty = 2;
                bot.game.reset_game(id);
                state.game_one.reset_game(id);
                ScreenState::VsBots
            },
            _ => ScreenState::GameModeSelector
        }
    }
}

pub fn handle_highscore_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, menuinputs : &MenuInputs){

    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(menuinputs.DOWN) || keyboard.is_key_just_pressed(menuinputs.UP){
        animation_state.selected_item_high_score.1 = (animation_state.selected_item_high_score.1 + 1) % 2;
    }else if keyboard.is_key_just_pressed(menuinputs.SELECT){
        if animation_state.selected_item_high_score.1  == 1{
            *screen_state = ScreenState::MainMenu;
        }
    }

    if animation_state.selected_item_high_score.1 == 0 {
        if keyboard.is_key_just_pressed(menuinputs.RIGHT) {
            animation_state.selected_item_high_score.0 = (animation_state.selected_item_high_score.0 + 1) % 3;
            animation_state.highscore_list = get_highscore_list(animation_state)
        }else if keyboard.is_key_just_pressed(menuinputs.LEFT) {
            animation_state.selected_item_high_score.0 = (animation_state.selected_item_high_score.0 + 2) % 3;
            animation_state.highscore_list = get_highscore_list(animation_state);
    
        }
    }
}

pub fn get_highscore_list(animation_state: &mut AnimationState) -> Vec<(String, usize)>{
    match animation_state.selected_item_high_score.0 {
        1 => get_scores_from_file("res/highscores/highscore_marathon.csv"),
        2 => get_scores_from_file("res/highscores/highscore_fourty_lines.csv"),
        0 => get_scores_from_file("res/highscores/highscore_survival.csv"),
        _ => Vec::new(),
    }
}

pub fn handle_name_inputs(
    ctx: &ggez::Context,
    _screen_state: &mut ScreenState,
    animation_state: &mut AnimationState,
    menuinputs : &MenuInputs
) {

    let keyboard = &ctx.keyboard;

    // Move cursor vertically
    if keyboard.is_key_just_pressed(menuinputs.DOWN) {
        if animation_state.selected_item_high_score.1 == 0 {
            // Move down in keyboard or go to CONTINUE
            if animation_state.selected_key.0 + 1 < 3 {
                animation_state.selected_key.0 += 1;
                animation_state.selected_key.1 = animation_state.selected_key.1.min(
                    get_keyboard_row(animation_state.selected_key.0).len() - 1,
                );
            } else {
                animation_state.selected_item_high_score.1 = 1; // CONTINUE
            }
        }
    } else if keyboard.is_key_just_pressed(menuinputs.UP) {
        if animation_state.selected_item_high_score.1 == 1 {
            animation_state.selected_item_high_score.1 = 0; 
        } else if animation_state.selected_key.0 > 0 {
            animation_state.selected_key.0 -= 1;
            animation_state.selected_key.1 = animation_state.selected_key.1.min(
                get_keyboard_row(animation_state.selected_key.0).len() - 1,
            );
        }
    }

    // Move cursor horizontally
    if animation_state.selected_item_high_score.1 == 0 {
        let row_len = get_keyboard_row(animation_state.selected_key.0).len();
        if keyboard.is_key_just_pressed(menuinputs.RIGHT) {
            animation_state.selected_key.1 = (animation_state.selected_key.1 + 1) % row_len;
        } else if keyboard.is_key_just_pressed(menuinputs.LEFT) {
            animation_state.selected_key.1 = (animation_state.selected_key.1 + row_len - 1) % row_len;
        }
    }

    // Select key or activate continue
    if keyboard.is_key_just_pressed(menuinputs.SELECT) {
        if animation_state.selected_item_high_score.1 == 1 {
            if animation_state.name_input != ""{
                animation_state.name_ready = true;
            }
        } else {
            let row = get_keyboard_row(animation_state.selected_key.0);
            if let Some(&ch) = row.get(animation_state.selected_key.1) {
                if animation_state.selected_key == (2,7){
                    animation_state.name_input.pop();
                }else {
                    if animation_state.name_input.len() < 10 {
                        animation_state.name_input.push(*ch);
                    }
                }
            }
        }
    }
}

// Helper to get keyboard rows
fn get_keyboard_row(row: usize) -> &'static [&'static char] {
    match row {
        0 => &[&'Q', &'W', &'E', &'R',& 'T',& 'Y',& 'U', &'I', &'O',& 'P',&'Å'],
        1 => &[&'A', &'S', &'D', &'F',& 'G',& 'H',& 'I', &'J', &'K',& 'L',&'Ö', &'Ä'],
        2 => &[&'Z', &'X', &'C', &'V',& 'B',& 'N',& 'M',&'<'],
        _ => &[],
    }
}

pub fn handle_settings_input(
    ctx: &ggez::Context,
    state : &mut AppState,
) {
    let keyboard = &ctx.keyboard;
    let animation_state = &mut state.animation_state; 
    let screen_state = &mut state.screen_state;
    let menuinputs = &state.menuinputs;

    // Move pointer
    if keyboard.is_key_just_pressed(menuinputs.DOWN) {
        animation_state.selected_item_settings.1 = (animation_state.selected_item_settings.1 + 1) % 4;
    }
    if keyboard.is_key_just_pressed(menuinputs.UP) {
        animation_state.selected_item_settings.1 = (animation_state.selected_item_settings.1 + 3) % 4;
    }
    if animation_state.selected_item_settings.1 == 3 {animation_state.edit_setting_value = false}

    if !animation_state.edit_setting_value 
    && (keyboard.is_key_just_pressed(menuinputs.LEFT) 
    || keyboard.is_key_just_pressed(menuinputs.RIGHT)) {
        animation_state.selected_item_settings.0 = (animation_state.selected_item_settings.0 + 1) % 2;
    }

    // Adjust selected setting
    if keyboard.is_key_just_released(menuinputs.LEFT)
    || keyboard.is_key_just_released(menuinputs.RIGHT) {
        animation_state.last_setting_tick = None;
    }

    if keyboard.is_key_just_pressed(menuinputs.SELECT)
    && animation_state.selected_item_settings.1 < 3 {
        animation_state.edit_setting_value = true
    }
    if keyboard.is_key_just_released(menuinputs.SELECT)
    && animation_state.selected_item_settings.1 < 3 {
        animation_state.edit_setting_value = false
    }

    let game_one = &mut state.game_one;
    let game_two = &mut state.game_two;

    if keyboard.is_key_pressed(menuinputs.LEFT) && animation_state.edit_setting_value {
        if animation_state.selected_item_settings.0 == 0 {
            if let Some(t) = animation_state.last_setting_tick {
                if t.elapsed().as_millis() >= SETTINGS_TICK_SPEED {
                    match animation_state.selected_item_settings.1 {
                        0 => decrement_das(game_one),
                        1 => decrement_arr(game_one),
                        2 => decrement_sds(game_one),
                        _ => ()
                    }
                    animation_state.last_setting_tick = Some(t + Duration::from_millis(SETTINGS_TICK_SPEED as u64))
                }
            } else {
                animation_state.last_setting_tick = Some(Instant::now() - Duration::from_millis(SETTINGS_TICK_SPEED as u64))
            }
        } else {
            if let Some(t) = animation_state.last_setting_tick {
                if t.elapsed().as_millis() >= SETTINGS_TICK_SPEED {
                    match animation_state.selected_item_settings.1 {
                        0 => decrement_das(game_two),
                        1 => decrement_arr(game_two),
                        2 => decrement_sds(game_two),
                        _ => ()
                    }
                    animation_state.last_setting_tick = Some(t + Duration::from_millis(SETTINGS_TICK_SPEED as u64))
                }
            } else {
                animation_state.last_setting_tick = Some(Instant::now() - Duration::from_millis(SETTINGS_TICK_SPEED as u64))
            }
        }
    }

    if keyboard.is_key_pressed(menuinputs.RIGHT) && animation_state.edit_setting_value {
        if animation_state.selected_item_settings.0 == 0 {
            if let Some(t) = animation_state.last_setting_tick {
                if t.elapsed().as_millis() >= SETTINGS_TICK_SPEED {
                    match animation_state.selected_item_settings.1 {
                        0 => increment_das(game_one),
                        1 => increment_arr(game_one),
                        2 => increment_sds(game_one),
                        _ => ()
                    }
                    animation_state.last_setting_tick = Some(t + Duration::from_millis(SETTINGS_TICK_SPEED as u64))
                }
            } else {
                animation_state.last_setting_tick = Some(Instant::now() - Duration::from_millis(SETTINGS_TICK_SPEED as u64))
            }
        } else {
            if let Some(t) = animation_state.last_setting_tick {
                if t.elapsed().as_millis() >= SETTINGS_TICK_SPEED {
                    match animation_state.selected_item_settings.1 {
                        0 => increment_das(game_two),
                        1 => increment_arr(game_two),
                        2 => increment_sds(game_two),
                        _ => ()
                    }
                    animation_state.last_setting_tick = Some(t + Duration::from_millis(SETTINGS_TICK_SPEED as u64))
                }
            } else {
                animation_state.last_setting_tick = Some(Instant::now() - Duration::from_millis(SETTINGS_TICK_SPEED as u64))
            }
        }
    }

    // Confirm
    if keyboard.is_key_just_pressed(menuinputs.SELECT)
    && animation_state.selected_item_settings.1 == 3 {
        *screen_state = ScreenState::MainMenu;
    }
}

// Helper functions
fn increment_das(game: &mut Game) {
    let das = game.das.as_millis();
    if das < 200 {
        game.das = Duration::from_millis((das + 5) as u64)
    }
}
fn decrement_das(game: &mut Game) {
    let das = game.das.as_millis();
    if das > 50 {
        game.das = Duration::from_millis((das - 5) as u64)
    }
}

fn increment_arr(game: &mut Game) {
    let arr = game.arr.as_millis();
    if arr < 100 {
        game.arr = Duration::from_millis((arr + 5) as u64)
    }
}
fn decrement_arr(game: &mut Game) {
    let arr = game.arr.as_millis();
    if arr > 0 {
        game.arr = Duration::from_millis((arr - 5) as u64)
    }
}

fn increment_sds(game: &mut Game) {
    let sds = game.sds;
    if sds < 100. {
        game.sds += 5.
    }
    else if sds == 100. {
        game.sds = INFINITY
    }
}
fn decrement_sds(game: &mut Game) {
    let sds = game.sds;
    if sds == INFINITY {
        game.sds = 100.
    }
    else if sds > 5. {
        game.sds -= 5.
    }
}