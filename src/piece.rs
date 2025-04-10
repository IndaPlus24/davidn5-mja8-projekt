use rand::{seq::SliceRandom, Rng};
use std::vec;

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

    pub fn get_random() -> PieceType {
        let mut rng = rand::rng();
        let piece_num = rng.random_range(0..7);
        PieceType::get_piecetype_from_num(piece_num)
    }

    pub fn get_piecetype_from_num(n: i32) -> PieceType {
        match n {
            0 => PieceType::I,
            1 => PieceType::O,
            2 => PieceType::T,
            3 => PieceType::S,
            4 => PieceType::Z,
            5 => PieceType::J,
            6 => PieceType::L,
            _ => PieceType::O,
        }
    }

    pub fn get_random_as_list() -> Vec<PieceType> {
        let mut rng = rand::rng();
        let mut nums: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6];
        nums.shuffle(&mut rng);

        let mut out: Vec<PieceType> = Vec::new();
        for n in nums {
            out.push(PieceType::get_piecetype_from_num(n));
        }
        out
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
