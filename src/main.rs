mod board;
mod config;
mod consts;
mod game;
mod game_ui;
mod inputs;
mod piece;
mod rotation;

use std::collections::HashMap;
use std::error::Error;
use std::path;
use std::str::FromStr;
use csv::{Reader, Writer};

pub use crate::config::input_config::*;
pub use crate::game::Game;
pub use crate::piece::{Piece, PieceType};
pub use crate::rotation::{ROTATION_180, ROTATION_CCW, ROTATION_CW};


use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};

struct AppState {
    images: HashMap<PieceType, Image>,
    game_one: Game,
    game_two : Game,
}

impl AppState {
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let mut state = AppState {
            images: AppState::preload_images(&ctx),
            game_one: Game::new(),
            game_two : Game::new(),
        };

        state.check_args();
        Ok(state)
    }

    pub fn preload_images(ctx: &Context) -> HashMap<PieceType, Image> {
        let mut image_map: HashMap<PieceType, Image> = HashMap::new();

        for i in 0..8 {
            let piece_type = PieceType::get_piecetype_from_num(i);
            let path = piece_type.get_path();
            let image = graphics::Image::from_path(ctx, path).unwrap();
            image_map.insert(piece_type, image);
        }

        image_map
    }

    pub fn check_args(&mut self){
        let args: Vec<String> = std::env::args().collect();

        //Sets controls to that of --drifarkaden 
        if args.contains(&"--drifarkaden".to_string()) {
            let drifar_keybinds: Vec<HashMap<GameAction, KeyCode>> = default_drivarkaden_keybindings();
            self.game_one.controls  = drifar_keybinds[0].clone();
            self.game_two.controls = drifar_keybinds[1].clone();
        } 
        
        //Runs the program in train ai mode. 
        else if args.contains(&"--train".to_string()) {
            //TODO -- train AI
        }
        
    }

    fn save_score(name : String, score : usize) -> Result<(), Box<dyn Error>> {

        println!("Saving score to file ...");
        let path = "res/highscore.csv";

        //Get previous scores and add new score
        let mut scores = get_scores_from_file(path);
        scores.push((name , score));
        
        //Sort scores and reverse
        scores.sort_by(|a,b| b.1.cmp(&a.1));

        //Write top 10 to file 
        if save_scores_to_file(path, scores){
            println!("Succesfully saved score to file ...");
        }
        Ok(())
    }
    
}

fn get_scores_from_file(path : &str) -> Vec<(String, usize)>{
    let mut rdr = Reader::from_path(path).expect("Couldn't open file");
        let mut scores: Vec<(String, usize)> = Vec::new();
    
        for result in rdr.records() {
            let record = result.expect("Couldn't parse result from file");
            scores.push((record.get(0).unwrap().to_string(),FromStr::from_str(record.get(1).unwrap()).expect("Score is not of type usize")));
        }
        scores
}

fn save_scores_to_file(path : &str, scores :Vec<(String, usize)>) -> bool{
    let mut wtr = Writer::from_path(path).expect("Couldn't open file");
    wtr.write_record(&["name", "score"]).expect("Couldnt write to file");  

    for (i,(n,s)) in scores.iter().enumerate(){
        if i == 10 {
            break;
        }
        let _ = wtr.write_record(&[n, &s.to_string()]);
    } 
    true
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game_one.next_tick(ctx);

        if self.game_one.game_over {
            //TODO Prompt name
            let name = "";
            let _ = Self::save_score(name.to_string(), self.game_one.score);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //CREATE CANVAS
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        //Render game
        self.game_one.render_game(&self.images, &mut canvas, ctx);

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
            conf::WindowMode::default().resizable(false), // Fixate window size
        );

    let (mut context, event_loop) = context_builder.build().expect("Failed to build context.");
    let state = AppState::new(&mut context).expect("Failed to create state.");

    println!("OPENED WINDOW");
    event::run(context, event_loop, state) // Run window event loop
}
