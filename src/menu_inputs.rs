use crate::{animation_state::AnimationState, consts::GameMode, get_scores_from_file, AppState, Game, GameAction, KeyCode, ScreenState};

// TODO: change key codes according to launch type
const UP: KeyCode = KeyCode::Up;
const DOWN: KeyCode = KeyCode::Down;
const LEFT: KeyCode = KeyCode::Left; 
const RIGHT: KeyCode = KeyCode::Right; 
const SELECT: KeyCode = KeyCode::Space;

pub fn handle_start_screen_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(SELECT) {
        *screen_state = ScreenState::MainMenu;
    }
}

pub fn handle_main_menu_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN) {
        animation_state.selected_item_main_menu = (animation_state.selected_item_main_menu + 1) % 3;
    } else if keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_main_menu = (animation_state.selected_item_main_menu + 2) % 3;
    } else if keyboard.is_key_just_pressed(SELECT) {
        *screen_state = match animation_state.selected_item_main_menu {
            0 => {
                animation_state.selected_item_gamemode_selector = 0;
                ScreenState::GameModeSelector
            }
            1 => {
                animation_state.selected_item_high_score = (0, 0);
                ScreenState::HighScore
            }
            _=> ScreenState::Settings
        };
    }
}

pub fn handle_gamemode_selector_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN) {
        animation_state.selected_item_gamemode_selector = (animation_state.selected_item_gamemode_selector + 1) % 4;
    } else if keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_gamemode_selector = (animation_state.selected_item_gamemode_selector + 3) % 4;
    } else if keyboard.is_key_just_pressed(SELECT) {
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

pub fn handle_singleplayer_selector_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, game: &mut Game) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN) {
        animation_state.selected_item_singleplayer_selector = (animation_state.selected_item_singleplayer_selector + 1) % 4;
    } else if keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_singleplayer_selector = (animation_state.selected_item_singleplayer_selector + 3) % 4;
    } else if keyboard.is_key_just_pressed(SELECT) {
        let selected = animation_state.selected_item_singleplayer_selector;
        *screen_state = match selected {
            0 => {
                animation_state.selected_item_marathon_prompt = (1, 0);
                ScreenState::MarathonPrompt
            }
            3 => ScreenState::GameModeSelector,
            _ => {
                game.reset_game();
                match selected {
                    1 => game.gamemode = GameMode::FourtyLines,
                    _ => game.gamemode = GameMode::Survival
                }
                ScreenState::Singleplayer
            }
        }
    }
}

pub fn handle_marathon_prompt_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, game: &mut Game) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN)
    || keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_marathon_prompt.1 = (animation_state.selected_item_marathon_prompt.1 + 1) % 2;
    } else if keyboard.is_key_just_pressed(LEFT) { // Decrease starting level
        let mut new_level = animation_state.selected_item_marathon_prompt.0 - 1;
        if new_level == 0 {new_level = 1}
        animation_state.selected_item_marathon_prompt.0 = new_level;
    } else if keyboard.is_key_just_pressed(RIGHT) { // Increase starting level
        let mut new_level = animation_state.selected_item_marathon_prompt.0 + 1;
        if new_level == 16 {new_level = 15}
        animation_state.selected_item_marathon_prompt.0 = new_level;
    } else if keyboard.is_key_just_pressed(SELECT) { // Set level and start game
        game.reset_game();
        game.set_level(animation_state.selected_item_marathon_prompt.0);
        game.gamemode = GameMode::Marathon;
        *screen_state = ScreenState::Singleplayer
    }
}

pub fn handle_reset_screen_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, game: &mut Game) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN)
    || keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_reset_selector = (animation_state.selected_item_reset_selector + 1) % 2;
    } else if keyboard.is_key_just_pressed(SELECT) {
        let selected = animation_state.selected_item_reset_selector;
        *screen_state = match selected {
            0 => {
                game.reset_game();
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
        game_one.reset_game();
        game_two.reset_game();
        *screen_state = ScreenState::Versus;
    }
}

pub fn handle_bot_selector_inputs(ctx: &ggez::Context, state : &mut AppState) {

    let animation_state = &mut state.animation_state;
    let screen_state = &mut state.screen_state;
    let bot = &mut state.bot;

    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN) {
        animation_state.selected_item_bot_selector = (animation_state.selected_item_bot_selector + 1) % 4;
    } else if keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_bot_selector = (animation_state.selected_item_bot_selector + 3) % 4;
    } else if keyboard.is_key_just_pressed(SELECT) {
        *screen_state = match animation_state.selected_item_bot_selector {
            0 => {
                bot.difficulty = 0;
                bot.game.reset_game();
                state.game_one.reset_game();
                ScreenState::VsBots
            },
            1 => {
                bot.difficulty = 1;
                bot.game.reset_game();
                state.game_one.reset_game();
                ScreenState::VsBots
            },
            2 => {
                bot.difficulty = 2;
                bot.game.reset_game();
                state.game_one.reset_game();
                ScreenState::VsBots
            },
            _ => ScreenState::GameModeSelector
        }
    }
}

pub fn handle_highscore_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState){

    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN) || keyboard.is_key_just_pressed(UP){
        animation_state.selected_item_high_score.1 = (animation_state.selected_item_high_score.1 + 1) % 2;
    }else if keyboard.is_key_just_pressed(SELECT){
        if animation_state.selected_item_high_score.1  == 1{
            *screen_state = ScreenState::MainMenu;
        }
    }

    if animation_state.selected_item_high_score.1 == 0 {
        if keyboard.is_key_just_pressed(RIGHT) {
            animation_state.selected_item_high_score.0 = (animation_state.selected_item_high_score.0 + 1) % 3;
            animation_state.highscore_list = get_highscore_list(animation_state)
        }else if keyboard.is_key_just_pressed(LEFT) {
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
) {

    let keyboard = &ctx.keyboard;

    // Move cursor vertically
    if keyboard.is_key_just_pressed(DOWN) {
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
    } else if keyboard.is_key_just_pressed(UP) {
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
        if keyboard.is_key_just_pressed(RIGHT) {
            animation_state.selected_key.1 = (animation_state.selected_key.1 + 1) % row_len;
        } else if keyboard.is_key_just_pressed(LEFT) {
            animation_state.selected_key.1 = (animation_state.selected_key.1 + row_len - 1) % row_len;
        }
    }

    // Select key or activate continue
    if keyboard.is_key_just_pressed(SELECT) {
        if animation_state.selected_item_high_score.1 == 1 {
            animation_state.name_ready = true;
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
    state : &mut AppState
) {
    let keyboard = &ctx.keyboard;
    let animation_state = &mut state.animation_state; 
    let screen_state = &mut state.screen_state;

    if keyboard.is_key_just_pressed(DOWN) {
        animation_state.selected_item_settings = (animation_state.selected_item_settings + 1) % 4;
    }
    if keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_settings = (animation_state.selected_item_settings + 3) % 4;
    }

    // Adjust selected setting
    let game = &mut state.game_one;

    match animation_state.selected_item_settings {
        0 => { // ARR
            if keyboard.is_key_pressed(LEFT) && game.arr.as_millis() > 0{
                let new_val = game.arr.as_millis().saturating_sub(1);
                game.arr = std::time::Duration::from_millis(new_val as u64);
            }
            if keyboard.is_key_pressed(RIGHT) &&game.arr.as_millis() < 999{
                let new_val = (game.arr + std::time::Duration::from_millis(1)).as_millis();
                game.arr = std::time::Duration::from_millis(new_val as u64);
            }
        }
        1 => { // DAS
            if keyboard.is_key_pressed(LEFT)&& game.das.as_millis() > 0 {
                let new_val = game.das.as_millis().saturating_sub(1);
                game.das = std::time::Duration::from_millis(new_val as u64);
            }
            if keyboard.is_key_pressed(RIGHT) && game.das.as_millis() < 999{
                let new_val = (game.das + std::time::Duration::from_millis(1)).as_millis();
                game.das = std::time::Duration::from_millis(new_val as u64);
            }
        }
        2 => { // SDS
            if keyboard.is_key_pressed(LEFT) && game.sds > 0.{
                let new_val = game.sds - 1.;
                game.sds = new_val;
            }
            if keyboard.is_key_pressed(RIGHT) && game.sds < 999.{
                let new_val = game.sds + 1.;
                game.sds = new_val;
            }
        }
        3 => { // Confirm
            if keyboard.is_key_just_pressed(SELECT) {
                *screen_state = ScreenState::MainMenu;
            }
        }
        _ => {}
    }

}
