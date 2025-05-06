use crate::{animation_state::AnimationState, bots::bot::Bot, consts::GameMode, Game, GameAction, KeyCode, ScreenState, get_scores_from_file};

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
            0 => ScreenState::GameModeSelector,
            1 => ScreenState::HighScore,
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
            1 => ScreenState::SingleplayerSelector,
            2 => ScreenState::BotSelector,
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
        match animation_state.selected_item_singleplayer_selector {
            0 => {
                *screen_state = ScreenState::Marathon;
                game.reset_game();
                game.gamemode = GameMode::Marathon;
                game.set_level(1);
            },
            1 => {
                *screen_state = ScreenState::FourtyLines;
                game.reset_game();
                game.gamemode = GameMode::FourtyLines;
            },
            2 => {
                *screen_state = ScreenState::Survival;
                game.reset_game();
            },
            _=> {
                *screen_state = ScreenState::GameModeSelector;
            }
        }
    }
}

pub fn handle_versus_ready_inputs(
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

pub fn handle_bot_selector_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState, bot : &mut Bot) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN) {
        animation_state.selected_item_bot_selector = (animation_state.selected_item_bot_selector + 1) % 4;
    } else if keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_bot_selector = (animation_state.selected_item_bot_selector + 3) % 4;
    } else if keyboard.is_key_just_pressed(SELECT) {
        *screen_state = match animation_state.selected_item_bot_selector {
            0 => {
                bot.difficulty = 0;
                ScreenState::VsBots
            },
            1 => {
                bot.difficulty = 1;
                ScreenState::VsBots
            },
            2 => {
                bot.difficulty = 2;
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
    use ggez::input::keyboard::KeyCode::*;

    let keyboard = &ctx.keyboard;

    // Move cursor vertically
    if keyboard.is_key_just_pressed(Down) {
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
    } else if keyboard.is_key_just_pressed(Up) {
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
        if keyboard.is_key_just_pressed(Right) {
            animation_state.selected_key.1 = (animation_state.selected_key.1 + 1) % row_len;
        } else if keyboard.is_key_just_pressed(Left) {
            animation_state.selected_key.1 = (animation_state.selected_key.1 + row_len - 1) % row_len;
        }
    }

    // Select key or activate continue
    if keyboard.is_key_just_pressed(Space) || keyboard.is_key_just_pressed(Return) {
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
