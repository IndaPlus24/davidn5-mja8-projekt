mod block;
mod board;
mod piece;

use std::collections::VecDeque;
use std::path;

pub use crate::block::{Block, BLOCK_SIZE, EMPTY_BLOCK_COLOR};
pub use crate::board::Board;
pub use crate::piece::{Piece, PieceType};

use ggez::input::keyboard::KeyCode;
use ggez::{conf, event, glam, graphics, Context, ContextBuilder, GameResult};

const BOARD_AMOUNT_COLUMNS: usize = 10;
const BOARD_AMOUNT_ROWS: usize = 20;
const BOARD_UPPER_LEFT: (i32, i32) = (100, 50);
const LEVELS_TICK_COUNTS: [u32; 1] = [60];

const TICKS_BETWEEN_INPUTS: usize = 2;
const GAME_TICKES_BEFORE_NEXT_PIECE: usize = 2;

const MOVE_PIECE_RIGHT: KeyCode = KeyCode::Right;
const MOVE_PIECE_LEFT: KeyCode = KeyCode::Left;
const MOVE_PIECE_DOWN_SOFT_DROP: KeyCode = KeyCode::Down;
const MOVE_PIECE_DOWN_HARD_DROP: KeyCode = KeyCode::Space;

struct AppState {
    tick_count: u32,
    current_level: usize,
    board: Board, // Board is a 20x10 of blocks
    active_piece: Piece,
    piece_queue: VecDeque<Piece>,
    ticks_since_last_input: usize,
    ticks_without_moving_down: usize,
}

impl AppState {
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let mut piece_queue: VecDeque<Piece> = VecDeque::new();
        let l = PieceType::get_random_as_list();
        for p in l {
            piece_queue.push_back(Piece::new(p));
        }

        let active_piece = piece_queue.pop_front().unwrap();

        let mut state = AppState {
            tick_count: 0,
            current_level: 0,
            board: Board::new(),
            active_piece: active_piece,
            piece_queue: piece_queue,
            ticks_since_last_input: 0,
            ticks_without_moving_down: 0,
        };

        for (r, c) in &state.active_piece.block_positions {
            state.board.table[*r][*c].path = state.active_piece.piece_type.get_path();
        }
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.tick_count += 1;
        self.ticks_since_last_input += 1;

        //Spawn new Piece
        if self.ticks_without_moving_down == GAME_TICKES_BEFORE_NEXT_PIECE {
            // Piece queue should have a set amount by default since it shows some of them to user
            if self.piece_queue.len() < 5 {
                //7-bag
                let l = PieceType::get_random_as_list();
                for p in l {
                    self.piece_queue.push_back(Piece::new(p));
                }
            }

            println!("Spawning new piece...");
            self.active_piece = self.piece_queue.pop_front().unwrap();
            self.ticks_without_moving_down = 0;
        }

        //CONTROLS

        if ctx.keyboard.is_key_pressed(MOVE_PIECE_RIGHT)
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.board.move_piece(&mut self.active_piece, 1, 0);
            self.ticks_since_last_input = 0;
        }

        if ctx.keyboard.is_key_pressed(MOVE_PIECE_LEFT)
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.board.move_piece(&mut self.active_piece, -1, 0);
            self.ticks_since_last_input = 0;
        }

        if ctx.keyboard.is_key_pressed(MOVE_PIECE_DOWN_SOFT_DROP)
            && self.ticks_since_last_input > TICKS_BETWEEN_INPUTS
        {
            self.board.move_piece(&mut self.active_piece, 0, 1);
            self.ticks_since_last_input = 0;
        }

        if ctx.keyboard.is_key_just_pressed(MOVE_PIECE_DOWN_HARD_DROP) {
            self.board.hard_drop(&mut self.active_piece);
            self.ticks_since_last_input = 0;
            //SPAWN A NEW PIECE IMMEDIETLY
            self.ticks_without_moving_down = GAME_TICKES_BEFORE_NEXT_PIECE;
            self.board.check_full_line(&self.active_piece);
        }

        // IF THE TICK COUNT MATCHES THE CURRENT LEVELS TICK COUNT
        if self.tick_count % LEVELS_TICK_COUNTS[self.current_level] == 0 {
            //MOVE PIECE DOWN
            if !self.board.move_piece(&mut self.active_piece, 0, 1) {
                self.ticks_without_moving_down += 1;
                println!("Piece at bottom...");
                //println!("Checking Lines...");
                self.board.check_full_line(&self.active_piece);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //CREATE CANVAS
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for r in 0..BOARD_AMOUNT_ROWS {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                if self.board.table[r][c].is_occupied() {
                    let image = graphics::Image::from_path(ctx, &self.board.table[r][c].path)?;
                    canvas.draw(
                        &image,
                        graphics::DrawParam::new().dest(glam::Vec2::new(
                            (BOARD_UPPER_LEFT.0 + c as i32 * BLOCK_SIZE + 1) as f32,
                            (BOARD_UPPER_LEFT.1 + r as i32 * BLOCK_SIZE + 1) as f32,
                        )),
                    );
                } else {
                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new_i32(
                            BOARD_UPPER_LEFT.0 + c as i32 * BLOCK_SIZE + 1,
                            BOARD_UPPER_LEFT.1 + r as i32 * BLOCK_SIZE + 1,
                            BLOCK_SIZE - 2,
                            BLOCK_SIZE - 2,
                        ),
                        EMPTY_BLOCK_COLOR,
                    )
                    .expect("COULDNT CREATE RECTANGLE FROM BLOCK");

                    canvas.draw(&rectangle, graphics::DrawParam::default());
                }
            }
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
            conf::WindowMode::default().resizable(false), // Fixate window size
        );

    let (mut contex, mut event_loop) = context_builder.build().expect("Failed to build context.");
    let state = AppState::new(&mut contex).expect("Failed to create state.");
    println!("OPENED WINDOW");
    event::run(contex, event_loop, state) // Run window event loop
}
