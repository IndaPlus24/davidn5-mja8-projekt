mod board;
mod config;
mod consts;
mod game;
mod game_ui;
mod inputs;
mod piece;
mod rotation;

use std::collections::HashMap;
use std::path;

pub use crate::config::input_config::*;
pub use crate::game::Game;
pub use crate::piece::{Piece, PieceType};
pub use crate::rotation::{ROTATION_180, ROTATION_CCW, ROTATION_CW};

use ggez::graphics::Image;
use ggez::input::keyboard::KeyCode;
use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};

struct AppState {
    images: HashMap<PieceType, Image>,
    game: Game,
}

impl AppState {
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let state = AppState {
            images: AppState::preload_images(&ctx),
            game: Game::new(),
        };
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
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game.next_tick(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //CREATE CANVAS
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        //Render game
        self.game.render_game(&self.images, &mut canvas, ctx);

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
