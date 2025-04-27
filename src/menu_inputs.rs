use crate::{animation_state::AnimationState, ScreenState, KeyCode};

// TODO: change key codes according to launch type
const UP: KeyCode = KeyCode::Up;
const DOWN: KeyCode = KeyCode::Down;
const LEFT: KeyCode = KeyCode::Left; // will be used for settings
const RIGHT: KeyCode = KeyCode::Right; // will be used for settings
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
        animation_state.selected_item_main_menu = 1
    } else if keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_main_menu = 0;
    } else if keyboard.is_key_just_pressed(SELECT) {
        *screen_state = match animation_state.selected_item_main_menu {
            0 => ScreenState::GameModeSelector,
            _ => ScreenState::Settings
        };
    }
}

pub fn handle_gamemode_selector_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN) {
        animation_state.selected_item_gamemode_selector = (animation_state.selected_item_gamemode_selector + 1) % 3;
    } else if keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_gamemode_selector = (animation_state.selected_item_gamemode_selector + 2) % 3;
    } else if keyboard.is_key_just_pressed(SELECT) {
        *screen_state = match animation_state.selected_item_gamemode_selector {
            0 => ScreenState::Multiplayer,
            1 => ScreenState::Singleplayer,
            _ => ScreenState::BotSelector,
        }
    }
}

pub fn handle_bot_selector_inputs(ctx: &ggez::Context, screen_state: &mut ScreenState, animation_state: &mut AnimationState) {
    let keyboard = &ctx.keyboard;
    if keyboard.is_key_just_pressed(DOWN) {
        animation_state.selected_item_bot_selector = (animation_state.selected_item_bot_selector + 1) % 3;
    } else if keyboard.is_key_just_pressed(UP) {
        animation_state.selected_item_bot_selector = (animation_state.selected_item_bot_selector + 2) % 3;
    } else if keyboard.is_key_just_pressed(SELECT) {
        *screen_state = match animation_state.selected_item_bot_selector {
            0 => ScreenState::Multiplayer,
            1 => ScreenState::Singleplayer,
            _ => ScreenState::VsBots,
        }
    }
}