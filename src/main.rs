mod animation_state;
mod board;
mod config;
mod consts;
mod game;
mod inputs;
mod piece;
mod rotation;
mod ui_components;

use consts::{GameState, GAME_1_POS, GAME_1_SCL, WINDOW_HEIGHT, WINDOW_WIDTH};
use csv::{Reader, Writer};
use std::collections::HashMap;
use std::error::Error;
use std::path;
use std::str::FromStr;
use ui_components::{bot_selecter, gamemode_selector, main_menu};

use crate::ui_components::start_screen;

pub use crate::config::input_config::*;
pub use crate::game::Game;
pub use crate::piece::{Piece, PieceType};
pub use crate::rotation::{ROTATION_180, ROTATION_CCW, ROTATION_CW};

use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};

struct AppState {
    piece_assets: HashMap<PieceType, Image>,
    board_assets: HashMap<String, Image>,
    menu_assets: HashMap<String, Image>,
    game_one: Game,
    game_two: Game,
}

impl AppState {
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let mut state = AppState {
            piece_assets: AppState::preload_piece_assets(ctx),
            board_assets: AppState::preload_board_assets(ctx),
            menu_assets: AppState::preload_menu_assets(ctx),
            game_one: Game::new(),
            game_two: Game::new(),
        };

        state.check_args();
        Ok(state)
    }

    pub fn preload_piece_assets(ctx: &Context) -> HashMap<PieceType, Image> {
        let mut image_map: HashMap<PieceType, Image> = HashMap::new();

        for i in 0..8 {
            let piece_type = PieceType::get_piecetype_from_num(i);
            let path = piece_type.get_path();
            let image = Image::from_path(ctx, path).unwrap();
            image_map.insert(piece_type, image);
        }

        image_map
    }

    pub fn preload_board_assets(ctx: &Context) -> HashMap<String, Image> {
        let mut image_map: HashMap<String, Image> = HashMap::new();

        image_map.insert(
            "main".to_string(),
            Image::from_path(ctx, "/board/main_board.png").unwrap(),
        );
        image_map.insert(
            "garb_bar".to_string(),
            Image::from_path(ctx, "/board/attack_bar.png").unwrap(),
        );
        image_map.insert(
            "garb_sep".to_string(),
            Image::from_path(ctx, "/board/attack_bar_seperator.png").unwrap(),
        );
        image_map.insert(
            "hold".to_string(),
            Image::from_path(ctx, "/board/hold.png").unwrap(),
        );

        image_map
    }

    pub fn preload_menu_assets(ctx: &Context) -> HashMap<String, Image> {
        let mut image_map: HashMap<String, Image> = HashMap::new();

        image_map.insert(
            "start_screen".to_string(),
            Image::from_path(ctx, "/ui_assets/start_screen.png").unwrap(),
        );
        image_map.insert(
            "empty_box".to_string(),
            Image::from_path(ctx, "/ui_assets/empty_box.png").unwrap(),
        );

        image_map
    }

    pub fn check_args(&mut self) {
        let args: Vec<String> = std::env::args().collect();

        //Sets controls to that of --drifarkaden
        if args.contains(&"--drifarkaden".to_string()) {
            let drifar_keybinds: Vec<HashMap<GameAction, KeyCode>> =
                default_drivarkaden_keybindings();
            self.game_one.controls = drifar_keybinds[0].clone();
            self.game_two.controls = drifar_keybinds[1].clone();
        }
        //Runs the program in train ai mode.
        else if args.contains(&"--train".to_string()) {
            //TODO -- train AI
        }
    }

    fn save_score(name: String, score: usize) -> Result<(), Box<dyn Error>> {
        println!("Saving score to file ...");
        let path = "res/highscore.csv";

        //Get previous scores and add new score
        let mut scores = get_scores_from_file(path);
        scores.push((name, score));

        //Sort scores and reverse
        scores.sort_by(|a, b| b.1.cmp(&a.1));

        //Write top 10 to file
        if save_scores_to_file(path, scores) {
            println!("Succesfully saved score to file ...");
        }
        Ok(())
    }
}

fn get_scores_from_file(path: &str) -> Vec<(String, usize)> {
    let mut rdr = Reader::from_path(path).expect("Couldn't open file");
    let mut scores: Vec<(String, usize)> = Vec::new();

    for result in rdr.records() {
        let record = result.expect("Couldn't parse result from file");
        scores.push((
            record.get(0).unwrap().to_string(),
            FromStr::from_str(record.get(1).unwrap()).expect("Score is not of type usize"),
        ));
    }
    scores
}

fn save_scores_to_file(path: &str, scores: Vec<(String, usize)>) -> bool {
    let mut wtr = Writer::from_path(path).expect("Couldn't open file");
    wtr.write_record(&["name", "score"])
        .expect("Couldnt write to file");

    for (i, (n, s)) in scores.iter().enumerate() {
        if i == 10 {
            break;
        }
        let _ = wtr.write_record(&[n, &s.to_string()]);
    }
    true
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.game_one.game_state {
            GameState::Singleplayer => {
                self.game_one.next_tick(ctx);
            }
            GameState::GameOver => {
                //TODO Prompt name
                let name = "";
                let _ = Self::save_score(name.to_string(), self.game_one.score);
                self.game_one.game_state = GameState::HighscoreInput;
            }
            GameState::StartScreen => {
                self.game_one.handle_start_screen_inputs(ctx);
            }
            GameState::MainMenu => {
                self.game_one.handle_main_menu_inputs(ctx);
            }
            GameState::GameModeSelector => {
                self.game_one.handle_gamemode_selector_inputs(ctx);
            }
            GameState::BotSelector => {
                self.game_one.handle_bot_selector_inputs(ctx);
            }
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        match self.game_one.game_state {
            GameState::Singleplayer => {
                //Render game
                self.game_one
                    .render_board(&self.board_assets, &mut canvas, GAME_1_POS, GAME_1_SCL);
                self.game_one.render_pieces(
                    &self.piece_assets,
                    &mut canvas,
                    GAME_1_POS,
                    GAME_1_SCL,
                );
            }
            GameState::StartScreen => {
                start_screen::render_start_screen(
                    &self.menu_assets,
                    &mut canvas,
                    ctx,
                    1.,
                    &mut self.game_one.animation_state,
                );
            }
            GameState::MainMenu => {
                main_menu::render_main_menu(
                    &self.menu_assets,
                    &mut canvas,
                    ctx,
                    1.,
                    &mut self.game_one.animation_state,
                );
            }
            GameState::GameModeSelector => {
                gamemode_selector::render_gamemode_selector(
                    &self.menu_assets,
                    &mut canvas,
                    ctx,
                    1.,
                    &mut self.game_one.animation_state,
                );
            }
            GameState::BotSelector => {
                bot_selecter::render_bot_selector(
                    &self.menu_assets,
                    &mut canvas,
                    ctx,
                    1.,
                    &mut self.game_one.animation_state,
                );
            }
            GameState::Multiplayer => {
                // If on keyboard switch controlls during the game and then switch back ...
                // since inputs for menus will be weird using multiplayer settings.
            }
            GameState::VsBots => {
                //Render 1v1 board but only load single player inputs that work on one of the boards.
            }
            _ => {}
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./res");

    let context_builder = ContextBuilder::new("Tetris", "davidn5, mja8")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("Tetris"))
        .window_mode(
            conf::WindowMode::default()
                .resizable(true)
                .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT),
        );

    let (mut context, event_loop) = context_builder.build().expect("Failed to build context.");
    let state = AppState::new(&mut context).expect("Failed to create state.");

    context.gfx.add_font(
        "Tetris font",
        graphics::FontData::from_path(&context, "/PressStart2P-Regular.ttf")?,
    );

    println!("Opened Window...");
    event::run(context, event_loop, state) // Run window event loop
}
