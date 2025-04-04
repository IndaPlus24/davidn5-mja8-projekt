use ggez::graphics::Color;

pub const BLOCK_SIZE: i32 = 25; 
pub const EMPTY_BLOCK_COLOR: Color = Color {
    g: 1.,
    b: 1.,
    r: 1.,
    a: 255.,
};
const BLOCK_COLORS: [Color; 3] = [Color::RED, Color::BLUE, Color::GREEN];

#[derive(Copy, Clone)]
pub struct Block {
    pub color: Color,
}

impl Block {
    pub fn new(c: Color) -> Self {
        Self { color: c }
    }
}