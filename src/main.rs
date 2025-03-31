use ggez::{
    conf, event,
    graphics::{self, Color},
    Context, ContextBuilder, GameResult,
};

const BOARD_AMOUNT_COLUMNS: usize = 10;
const BOARD_AMOUNT_ROWS: usize = 20;
const BOARD_UPPER_LEFT: (i32, i32) = (100, 50);
const BLOCK_SIZE: i32 = 25;

struct AppState {
    board: [[Block; BOARD_AMOUNT_COLUMNS]; BOARD_AMOUNT_ROWS], // Board is a 10 x 20 of blocks
}
#[derive(Copy, Clone)]
struct Block {
    color: Color,
}

impl Block {
    fn new(c: Color) -> Self {
        Self { color: c }
    }
}

impl AppState {
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        let mut state = AppState {
            board: [[Block::new(Color::from_rgb(59, 59, 59)); BOARD_AMOUNT_COLUMNS];
                BOARD_AMOUNT_ROWS],
        };
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for AppState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
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
                    self.board[r][c].color,
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
