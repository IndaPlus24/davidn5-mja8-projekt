mod piece;
mod block;

pub use crate::block::{Block,BLOCK_SIZE,EMPTY_BLOCK_COLOR};
pub use crate::piece::{Piece,PieceType};

use ggez::graphics::Color;
use ggez::{
    conf, event,
    graphics,
    Context, ContextBuilder, GameResult,
};

const BOARD_AMOUNT_COLUMNS: usize = 10; 
const BOARD_AMOUNT_ROWS: usize = 20;
const BOARD_UPPER_LEFT: (i32, i32) = (100, 50);
const LEVEL_ONE_TICK_COUNT_GRAVITY : u32 = 60;

struct AppState {
    tick_count : u32,
    board: [[Block; BOARD_AMOUNT_ROWS]; BOARD_AMOUNT_COLUMNS], // Board is a 10 x 20 of blocks
    active_piece : Piece
}


impl AppState {
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let mut state = AppState {
            tick_count : 0,
            board: [[Block::new(EMPTY_BLOCK_COLOR); BOARD_AMOUNT_ROWS]; BOARD_AMOUNT_COLUMNS],
            active_piece : Piece::new(PieceType::O) // TODO GOTTA MAKE THE PIECE TYPE RANDOM
        };

        for (c,r) in &state.active_piece.block_positions{
            state.board[*c][*r].color = state.active_piece.piece_type.color();
        } 
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.tick_count += 1;

        if self.tick_count % LEVEL_ONE_TICK_COUNT_GRAVITY == 0 {
            let mut can_move_down = true;
            for (c,r) in &self.active_piece.block_positions {
                if r + 1 < BOARD_AMOUNT_ROWS{
                    if self.board[*c][r + 1].is_occupied() {
                        can_move_down = false;
                    }
                }else {can_move_down = false}
            }

            if can_move_down {
                let old_positions = self.active_piece.block_positions.clone();

                for (i,( c,r )) in old_positions.iter().enumerate()  {
                    self.board[*c][*r].color =  EMPTY_BLOCK_COLOR;
                    self.board[*c][r+1].color = self.active_piece.piece_type.color();
                    self.active_piece.block_positions[i] = (*c,r + 1);
                }
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
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new_i32(
                        BOARD_UPPER_LEFT.0 + c as i32 * BLOCK_SIZE + 1,
                        BOARD_UPPER_LEFT.1 + r as i32 * BLOCK_SIZE + 1,
                        BLOCK_SIZE - 2,
                        BLOCK_SIZE - 2,
                    ),
                    self.board[c][r].color,
                )
                .expect("COULDNT CREATE RECTANGLE FROM BLOCK");

                canvas.draw(&rectangle, graphics::DrawParam::default());
            }
        }


    
        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let context_builder = ContextBuilder::new("Tetris", "davidn5, mja8")
        .window_setup(conf::WindowSetup::default().title("Tetris"))
        .window_mode(
            conf::WindowMode::default().resizable(false), // Fixate window size
        );

    let (mut contex, mut event_loop) = context_builder.build().expect("Failed to build context.");
    let state = AppState::new(&mut contex).expect("Failed to create state.");
    println!("OPENED WINDOW");
    event::run(contex, event_loop, state) // Run window event loop
}
