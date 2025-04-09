use ggez::graphics::Color;

pub const BLOCK_SIZE: i32 = 25;
pub const EMPTY_BLOCK_COLOR: Color = Color {
    g: 1.,
    b: 1.,
    r: 1.,
    a: 255.,
};

#[derive(Clone, Default)]
pub struct Block {
    pub occupied: bool,
    pub path: String,
}

impl Block {
    pub fn new() -> Self {
        Self {
            occupied: false,
            path: "test".to_string(),
        }
    }

    pub fn is_occupied(&self) -> bool {
        self.occupied
    }
}
