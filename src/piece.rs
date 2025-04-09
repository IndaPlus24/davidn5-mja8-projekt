use std::vec;

use ggez::graphics::Color;

#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}
impl PieceType {
    pub fn get_path(&self) -> String {
        match self {
            PieceType::I => String::from("/cyan.png"),
            PieceType::O => String::from("/yellow.png"),
            PieceType::T => String::from("/magenta.png"),
            PieceType::S => String::from("/green.png"),
            PieceType::Z => String::from("/red.png"),
            PieceType::J => String::from("/blue.png"),
            PieceType::L => String::from("/orange.png"),
        }
    }
}

pub struct Piece {
    pub piece_type: PieceType,
    pub block_positions: Vec<(usize, usize)>, // An array of tuples with the position of the pieces blocks
    pub is_active: bool,                      // active piece is the piece that can be moved.
}

impl Piece {
    // ALL PIECE TYPE POSITIONS ARE WRITTEN WITH "LOWEST" BLOCK FIRST
    fn get_block_positions(piece_type: PieceType) -> Vec<(usize, usize)> {
        match piece_type {
            PieceType::I => vec![(0, 3), (0, 4), (0, 5), (0, 6)], // WRITTEN (R, C)
            PieceType::J => vec![(1, 3), (1, 4), (1, 5), (0, 3)],
            PieceType::L => vec![(1, 3), (0, 3), (0, 4), (0, 5)],
            PieceType::O => vec![(1, 4), (1, 5), (0, 4), (0, 5)],
            PieceType::S => vec![(1, 3), (1, 4), (0, 4), (0, 5)],
            PieceType::Z => vec![(1, 4), (1, 5), (0, 3), (0, 4)],
            PieceType::T => vec![(1, 4), (0, 3), (0, 4), (0, 5)],
        }
    }

    pub fn new(piece_type: PieceType) -> Self {
        let blocks: Vec<(usize, usize)> = Piece::get_block_positions(piece_type);

        Self {
            piece_type: piece_type,
            block_positions: blocks,
            is_active: true,
        }
    }
}
