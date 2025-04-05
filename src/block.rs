use ggez::graphics::Color;

pub const BLOCK_SIZE: i32 = 25; 
pub const EMPTY_BLOCK_COLOR: Color = Color {
    g: 1.,
    b: 1.,
    r: 1.,
    a: 255.,
};

#[derive(Copy, Clone)]
pub struct Block {
    pub color: Color,
    pub occupied : bool
}

impl Block {
    pub fn new(c: Color) -> Self {
        Self { color: c, occupied : false }
    }

    pub fn is_occupied (&self) -> bool {
        self.occupied
    } 
}