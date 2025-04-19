mod block;
mod board;
mod piece;
mod rotation;
mod inputs;
mod config;
mod game;
mod game_ui;
mod consts;

use std::collections::HashMap;
use std::path;

pub use crate::block::Block;
pub use crate::game::Game;
pub use crate::board::Board;
pub use crate::piece::{Piece, PieceType};
pub use crate::rotation::{ROTATION_CW, ROTATION_CCW, ROTATION_180};
pub use crate::config::input_config::*;

use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};


struct AppState {
    images : HashMap<String, Image>,
    game : Game,
}

impl AppState {
    fn new(ctx: &mut Context) -> GameResult<AppState> {

        let state = AppState {
            images: AppState::preload_images(&ctx),
            game : Game::new(),
        };
        Ok(state)
    }

    pub fn preload_images(ctx : &Context) -> HashMap<String, Image>{
        let mut image_map : HashMap<String, Image> = HashMap::new(); 
    
        for i in 0..8 {
            let piece_type = PieceType::get_piecetype_from_num(i);
            let path = piece_type.get_path();
            let image = graphics::Image::from_path(ctx, path).unwrap();
            image_map.insert(piece_type.get_path(), image);
        }
    
        image_map
    }

    
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        self.game.next_tick(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //CREATE CANVAS
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        //Render board
        self.game.render_board(&self.images, &mut canvas, ctx);

        //Render Ghost Piece
        let image = self.images.get("/grey.png").unwrap(); 
        self.game.render_ghost_piece(image, &mut canvas);

        //Render active piece
        let path = &self.game.active_piece.piece_type.get_path();
        let image = self.images.get(path).unwrap();
        self.game.render_active_piece(image, &mut canvas);

        //Render the held piece (if it exists)
        self.game.render_held_piece(&self.images,&mut canvas);

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
