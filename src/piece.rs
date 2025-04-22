use rand::{seq::SliceRandom, Rng};
use std::vec;

use crate::rotation::{
    RELATIVE_MINOS_I, RELATIVE_MINOS_J, RELATIVE_MINOS_L, RELATIVE_MINOS_O, RELATIVE_MINOS_S,
    RELATIVE_MINOS_T, RELATIVE_MINOS_Z,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
    X,
}
impl PieceType {
    pub fn get_path(&self) -> String {
        match self {
            PieceType::I => String::from("/pieces/cyan.png"),
            PieceType::J => String::from("/pieces/blue.png"),
            PieceType::L => String::from("/pieces/orange.png"),
            PieceType::O => String::from("/pieces/yellow.png"),
            PieceType::S => String::from("/pieces/green.png"),
            PieceType::T => String::from("/pieces/magenta.png"),
            PieceType::Z => String::from("/pieces/red.png"),
            PieceType::X => String::from("/pieces/grey.png"),
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
            1 => PieceType::J,
            2 => PieceType::L,
            3 => PieceType::O,
            4 => PieceType::S,
            5 => PieceType::T,
            6 => PieceType::Z,
            _ => PieceType::X,
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

#[derive(Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub midpoint: (isize, isize), // Tuple with the piece midpoint in the board (R, C)
    pub block_positions: Vec<(isize, isize)>, // An array of tuples with the mino positions relative to the tetromino's midpoint
    pub rotation: usize,
}

impl Piece {
    // ALL PIECE TYPE POSITIONS ARE RELATIVE TO THE MIDPOINT
    pub fn get_block_positions(piece_type: PieceType, rotation: usize) -> Vec<(isize, isize)> {
        match piece_type {
            PieceType::I => RELATIVE_MINOS_I[rotation].to_vec(),
            PieceType::J => RELATIVE_MINOS_J[rotation].to_vec(),
            PieceType::L => RELATIVE_MINOS_L[rotation].to_vec(),
            PieceType::O => RELATIVE_MINOS_O[rotation].to_vec(),
            PieceType::S => RELATIVE_MINOS_S[rotation].to_vec(),
            PieceType::T => RELATIVE_MINOS_T[rotation].to_vec(),
            PieceType::Z => RELATIVE_MINOS_Z[rotation].to_vec(),
            PieceType::X => Vec::new(),
        }
    }

    pub fn new(piece_type: PieceType, rotation: usize) -> Self {
        let blocks: Vec<(isize, isize)> = Piece::get_block_positions(piece_type, rotation);

        Self {
            piece_type: piece_type,
            midpoint: (20, 4),
            block_positions: blocks,
            rotation: 0,
        }
    }
}
