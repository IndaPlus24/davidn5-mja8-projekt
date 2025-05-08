mod animation_state;
mod board;
mod bots;
mod config;
mod consts;
mod game;
mod game_inputs;
mod menu_inputs;
mod piece;
mod rotation;
mod scoring;
mod ui_components;
mod gamemodes;

use animation_state::AnimationState;
use bots::bot::Bot;
use bots::train_bot::train_ai;
use consts::*;
use csv::{Reader, Writer};
use menu_inputs::*;
use rand::{random_range, Rng};
use std::collections::HashMap;
use std::error::Error;
use std::path;
use std::str::FromStr;
use std::time::{Duration, Instant};
use ui_components::*;

pub use crate::config::input_config::*;
pub use crate::game::Game;
pub use crate::piece::{Piece, PieceType};
pub use crate::rotation::{ROTATION_180, ROTATION_CCW, ROTATION_CW};

use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};

struct AppState {
    //Screen states and info
    animation_state: AnimationState,
    screen_state: ScreenState,
    drifarkaden: bool,

    // Assets
    piece_assets: HashMap<PieceType, Image>,
    board_assets: HashMap<String, Image>,
    menu_assets: HashMap<String, Image>,
    misc_assets: HashMap<String, Image>,

    timer: Option<Instant>,

    // Games
    game_one: Game,
    game_two: Game,

    // Bots,
    bot: Bot,

    menuinputs : MenuInputs,
}

impl AppState {
    fn new(
        ctx: &mut Context,
        args: Option<Vec<HashMap<GameAction, KeyCode>>>,
    ) -> GameResult<AppState> {
        let mut rng = rand::rng();
        let id = Some(rng.random());

        let mut state = AppState {
            animation_state: AnimationState::new(get_scores_from_file("res/highscores/highscore_marathon.csv")),
            screen_state: ScreenState::StartScreen,
            drifarkaden: false,

            piece_assets: AppState::preload_piece_assets(ctx),
            board_assets: AppState::preload_board_assets(ctx),
            menu_assets: AppState::preload_menu_assets(ctx),
            misc_assets: AppState::preload_misc_assets(ctx),

            timer: None,

            game_one: Game::new(GAME_1_SOLO_POS, GAME_1_SOLO_SCL, id.unwrap()),
            game_two: Game::new(GAME_2_VS_POS, GAME_2_VS_SCL, id.unwrap()),

            bot: Bot::new(0, id.unwrap()),
            menuinputs : MenuInputs::pc_inputs()
        };

        state.bot.game.canvas_pos = GAME_2_VS_POS; 
        state.bot.game.canvas_scl = GAME_2_VS_SCL;

        state.game_one.reset_game(id);
        state.game_two.reset_game(id);

        if let Some(keybinds) = args {
            state.game_one.controls = keybinds[0].clone();
            state.game_two.controls = keybinds[1].clone();
            state.drifarkaden = true;
            state.menuinputs = MenuInputs::drifarkaden_inputs();
        }

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

        image_map.insert( // Attack of 1
            "garb_s".to_string(),
            Image::from_path(ctx, "/board/garbage_s.png").unwrap(),
        );
        image_map.insert( // Top part of attack > 1
            "garb_t".to_string(),
            Image::from_path(ctx, "/board/garbage_t.png").unwrap(),
        );
        image_map.insert( // Middle part of attack > 1
            "garb_m".to_string(),
            Image::from_path(ctx, "/board/garbage_m.png").unwrap(),
        );
        image_map.insert( // Bottom part of attack > 1
            "garb_b".to_string(),
            Image::from_path(ctx, "/board/garbage_b.png").unwrap(),
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

    pub fn preload_misc_assets(ctx: &Context) -> HashMap<String, Image> {
        let mut image_map: HashMap<String, Image> = HashMap::new();

        image_map.insert(
            "finish".to_string(),
            Image::from_path(ctx, "/misc/finish.png").unwrap(),
        );
        image_map.insert(
            "game_over".to_string(),
            Image::from_path(ctx, "/misc/game_over.png").unwrap(),
        );
        image_map.insert(
            "line_marker".to_string(),
            Image::from_path(ctx, "/misc/line_goal_marker.png").unwrap(),
        );

        image_map
    }

    fn save_score(name: String, score: usize, path: &str) -> Result<(), Box<dyn Error>> {
        //Get previous scores and add new score
        let mut scores = get_scores_from_file(path);
        scores.push((name, score));

        //Sort scores and reverse
        scores.sort_by(|a, b| b.1.cmp(&a.1));

        //Write top 5 to file
        if save_scores_to_file(path, scores) {
            //println!("Succesfully saved score to file ...");
        }
        Ok(())
    }
}

pub fn get_scores_from_file(path: &str) -> Vec<(String, usize)> {
    let mut rdr = Reader::from_path(path).expect("Couldn't open file");
    let mut scores: Vec<(String, usize)> = Vec::new();

    for result in rdr.records() {
        let record = result.expect("Couldn't parse result from file");
        scores.push((
            record.get(0).unwrap().to_string(),
            FromStr::from_str(record.get(1).unwrap()).expect("Score is not of type usize"),
        ));
    }
    while scores.len() < 5 {
        scores.push(("empty".to_string(),0));
    }
    scores
}

pub fn save_scores_to_file(path: &str, scores: Vec<(String, usize)>) -> bool {
    let mut wtr = Writer::from_path(path).expect("Couldn't open file");
    wtr.write_record(&["name", "score"])
        .expect("Couldnt write to file");

    for (i, (n, s)) in scores.iter().enumerate() {
        if i == 5 {
            break;
        }
        let _ = wtr.write_record(&[n, &s.to_string()]);
    }
    true
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        if self.game_one.game_over && self.game_one.continue_to_highscore {
            self.screen_state = ScreenState::HighscoreInput;
            
            if self.animation_state.name_ready {
                let name = &self.animation_state.name_input;
                let path = match self.game_one.gamemode {
                    GameMode::Marathon => "res/highscores/highscore_marathon.csv",
                    GameMode::FourtyLines => "res/highscores/highscore_fourty_lines.csv",
                    _ => "res/highscores/highscore_survival.csv"
                };
                if self.game_one.gamemode == GameMode::FourtyLines{
                    let _ = Self::save_score(name.to_string(), self.game_one.final_time.as_secs() as usize, path);
                }else{
                    let _ = Self::save_score(name.to_string(), self.game_one.score, path);
                }
                self.animation_state.name_input = "".to_string();
                self.animation_state.name_ready = false;

                self.game_one.game_over = false;
                self.game_one.continue_to_highscore = false;
                self.screen_state = ScreenState::HighScore;
            }
        }
        match self.screen_state {
            ScreenState::Singleplayer => {
                self.game_one.update(ctx);

                // 40L top-out check
                if self.game_one.gamemode == GameMode::FourtyLines
                && self.game_one.game_over
                && !self.game_one.objective_completed {
                    self.animation_state.selected_item_reset_selector = 0;
                    self.screen_state = ScreenState::FourtyLinesReset
                }

                // Survival garbage
                if self.game_one.gamemode == GameMode::Survival && !self.game_one.game_over && self.game_one.countdown_start.is_none() {
                    if let Some(t) = self.timer {
                        if t.elapsed() >= Duration::from_millis(SURVIVAL_TIMER) {
                            self.timer = Some(t + Duration::from_secs(1));
                            self.game_one.add_garbage_row(random_range(0..9));
                        }
                    } else {
                        self.timer = Some(Instant::now())
                    }
                }
            }
            ScreenState::StartScreen => {
                handle_start_screen_inputs(ctx, &mut self.screen_state, &self.menuinputs);
            }
            ScreenState::MainMenu => {
                handle_main_menu_inputs(ctx, &mut self.screen_state, &mut self.animation_state, &self.menuinputs);
            }
            ScreenState::GameModeSelector => {
                handle_gamemode_selector_inputs(ctx, &mut self.screen_state, &mut self.animation_state, &self.menuinputs);
            }
            ScreenState::SingleplayerSelector => {
                handle_singleplayer_selector_inputs(ctx, &mut self.screen_state, &mut self.animation_state, &mut self.game_one, &self.menuinputs);
            }
            ScreenState::MarathonPrompt => {
                handle_marathon_prompt_inputs(ctx, &mut self.screen_state, &mut self.animation_state, &mut self.game_one, &self.menuinputs);
            }
            ScreenState::FourtyLinesReset => {
                handle_reset_screen_inputs(ctx, &mut self.screen_state, &mut self.animation_state, &mut self.game_one,&self.menuinputs);
            }
            ScreenState::Settings => {
                handle_settings_input(ctx, self);
            }

            // Versus
            ScreenState::VersusReady => {
                if self.game_one.gamemode != GameMode::Versus
                || self.game_two.gamemode != GameMode::Versus {
                    if !self.drifarkaden {
                        let vs_controls = multi_controller_keyboard_keybindings();
                        self.game_one.controls = vs_controls[0].clone();
                        self.game_two.controls = vs_controls[1].clone();
                    }

                    self.game_one.canvas_pos = GAME_1_VS_POS;
                    self.game_one.canvas_scl = GAME_1_VS_SCL;
                    
                    self.game_one.gamemode = GameMode::Versus;
                    self.game_two.gamemode = GameMode::Versus;
                }

                handle_versus_prepost_inputs(
                    ctx,
                    &mut self.screen_state,
                    &mut self.animation_state,
                    &mut self.game_one,
                    &mut self.game_two,
                )
            }
            ScreenState::Versus => {
                self.game_one.update(ctx);
                self.game_two.update(ctx);

                // Garbage handling
                while self.game_one.garbage_outbound.len() > 0 {
                    self.game_two.receive_garbage(
                        self.game_one.garbage_outbound.pop_front().unwrap()
                    );
                }
                while self.game_two.garbage_outbound.len() > 0 {
                    self.game_one.receive_garbage(
                        self.game_two.garbage_outbound.pop_front().unwrap()
                    );
                }

                if self.game_one.game_over || self.game_two.game_over {
                    self.animation_state.players_ready = (false, false);
                    self.timer = Some(Instant::now());
                    self.screen_state = ScreenState::VersusRematch;
                }
            }
            ScreenState::VersusRematch => {
                if let Some(t) = self.timer {
                    if t.elapsed() >= Duration::from_secs(10) {
                        if !self.drifarkaden {
                            self.game_one.controls = default_keyboard_keybindings();
                        }

                        self.game_one.canvas_pos = GAME_1_SOLO_POS;
                        self.game_one.canvas_scl = GAME_1_SOLO_SCL;

                        self.screen_state = ScreenState::MainMenu;
                    }
                }

                handle_versus_prepost_inputs(
                    ctx,
                    &mut self.screen_state,
                    &mut self.animation_state,
                    &mut self.game_one,
                    &mut self.game_two,
                );
            }

            ScreenState::BotSelector => {
                handle_bot_selector_inputs(ctx,self);
            }
            ScreenState::VsBots => {

                if self.game_one.gamemode != GameMode::Versus{
                    if !self.drifarkaden {
                        self.game_one.controls = default_keyboard_keybindings();
                    }

                    self.game_one.canvas_pos = GAME_1_VS_POS;
                    self.game_one.canvas_scl = GAME_1_VS_SCL;
                    
                    self.game_one.gamemode = GameMode::Versus;
                    self.bot.game.gamemode = GameMode::Versus;
                }

                if self.game_one.game_over || self.bot.game.game_over {
                    if ctx.keyboard.is_key_just_pressed(*self.game_one.controls.get(&GameAction::HardDrop).unwrap()) {
                        self.screen_state = ScreenState::MainMenu;
                    }
                }else {
                    self.game_one.update(ctx);
                    self.bot.render_bot_game(ctx);
                }

                // Garbage handling
                while self.game_one.garbage_outbound.len() > 0 {
                    self.bot.game.receive_garbage(
                        self.game_one.garbage_outbound.pop_front().unwrap()
                    );
                }
                while self.bot.game.garbage_outbound.len() > 0 {
                    self.game_one.receive_garbage(
                        self.bot.game.garbage_outbound.pop_front().unwrap()
                    );
                }

            }
            ScreenState::HighScore => {
                handle_highscore_inputs(ctx, &mut self.screen_state, &mut self.animation_state, &self.menuinputs);
            }
            ScreenState::HighscoreInput => {
                handle_name_inputs(ctx, &mut self.screen_state, &mut self.animation_state, &self.menuinputs);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        match &self.screen_state {
            ScreenState::Singleplayer => {
                //Render game
                self.game_one
                    .render_board(&self.board_assets, &mut canvas)
                    .render_pieces(&self.piece_assets, &mut canvas)
                    .render_stats(&mut canvas)
                    .render_misc(&self.misc_assets, &mut canvas);
            }
            ScreenState::StartScreen => {
                start_screen::render_start_screen(
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    &mut self.animation_state,
                    self.drifarkaden
                );
            }
            ScreenState::MainMenu => {
                main_menu::render_main_menu(
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    &mut self.animation_state,
                );
            }
            ScreenState::GameModeSelector => {
                gamemode_selector::render_gamemode_selector(
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    &mut self.animation_state,
                );
            }
            ScreenState::SingleplayerSelector => {
                singleplayer_selector::render_gamemode_selector(
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    &mut self.animation_state,
                );
            }
            ScreenState::MarathonPrompt => {
                marathon_prompt::render_marathon_prompt(
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    &mut self.animation_state,
                );
            }
            ScreenState::FourtyLinesReset => {
                reset_screen::render_reset_screen(
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    &mut self.animation_state,
                );
            }
            ScreenState::Settings => {
                settings::render_settings(
                    &mut canvas, 
                    1.,
                    self
                );
            }

            // Versus
            ScreenState::VersusReady => {
                versus_ready::render_versus_ready(
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    &mut self.animation_state,
                );
            }
            ScreenState::Versus => {
                self.game_one
                    .render_board(&self.board_assets, &mut canvas)
                    .render_pieces(&self.piece_assets, &mut canvas)
                    .render_stats(&mut canvas)
                    .render_misc(&self.misc_assets, &mut canvas);
                
                self.game_two
                    .render_board(&self.board_assets, &mut canvas)
                    .render_pieces(&self.piece_assets, &mut canvas)
                    .render_stats(&mut canvas)
                    .render_misc(&self.misc_assets, &mut canvas);
            }
            ScreenState::VersusRematch => {
                let winner = if self.game_one.game_over {1} else {0};

                versus_rematch::render_versus_rematch(
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    &mut self.animation_state,
                    winner,
                    self.timer,
                );
            }

            ScreenState::BotSelector => {
                bot_selector::render_bot_selector(
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    &mut self.animation_state,
                );
            }
            ScreenState::HighScore => {
                high_score::render_high_score(
                    &self,
                    &self.menu_assets,
                    &mut canvas,
                    1.,
                    );
            }
            ScreenState::HighscoreInput => {
                input_name::render_input_name(
                    self,
                    &mut canvas,
                    1.,
                    );
            }
            ScreenState::VsBots => {
                //Render game
                self.bot.game
                    .render_board(&self.board_assets, &mut canvas)
                    .render_pieces(&self.piece_assets,&mut canvas)
                    .render_stats(&mut canvas)
                    .render_misc(&self.misc_assets, &mut canvas);


                self.game_one
                    .render_board(&self.board_assets, &mut canvas)
                    .render_pieces(&self.piece_assets, &mut canvas)
                    .render_stats(&mut canvas)
                    .render_misc(&self.misc_assets, &mut canvas);
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn check_args() -> Option<Vec<HashMap<GameAction, KeyCode>>> {
    let args: Vec<String> = std::env::args().collect();

    //Sets controls to that of --drifarkaden
    if args.contains(&"--drifarkaden".to_string()) {
        return Some(default_drivarkaden_keybindings());
    }
    //Runs the program in train ai mode.
    else if args.contains(&"--train".to_string()) {
        train_ai();
    }
    None
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

    let args = check_args();

    let (mut context, event_loop) = context_builder.build().expect("Failed to build context.");
    let state = AppState::new(&mut context, args).expect("Failed to create state.");

    context.gfx.add_font(
        "Tetris font",
        graphics::FontData::from_path(&context, "/PressStart2P-Regular.ttf")?,
    );

    event::run(context, event_loop, state) // Run window event loop
}
