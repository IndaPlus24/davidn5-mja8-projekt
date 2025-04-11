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
    pub midpoint: (isize, isize), // Tuple with the piece midpoint in the board (R, C)
    pub block_positions: Vec<(isize, isize)>, // An array of tuples with the mino positions relative to the tetromino's midpoint
    pub is_active: bool,                      // active piece is the piece that can be moved.
}

impl Piece {
    // ALL PIECE TYPE POSITIONS ARE RELATIVE TO THE MIDPOINT
    fn get_block_positions(piece_type: PieceType) -> Vec<(isize, isize)> {
        match piece_type {
            PieceType::I => vec![( 0, -1), ( 0,  0), (0,  1), (0, 2)], // WRITTEN (DR, DC)
            PieceType::J => vec![(-1, -1), ( 0, -1), (0,  0), (0, 1)],
            PieceType::L => vec![(-1,  1), ( 0, -1), (0,  0), (0, 1)],
            PieceType::O => vec![(-1,  0), (-1,  1), (0,  0), (0, 1)],
            PieceType::S => vec![(-1,  0), (-1,  1), (0, -1), (0, 0)],
            PieceType::Z => vec![(-1, -1), (-1,  0), (0,  0), (0, 1)],
            PieceType::T => vec![(-1,  0), ( 0, -1), (0,  0), (0, 1)],
        }
    }

    pub fn new(piece_type: PieceType) -> Self {
        let blocks: Vec<(isize, isize)> = Piece::get_block_positions(piece_type);

        Self {
            piece_type: piece_type,
            midpoint: (-1, 4),
            block_positions: blocks,
            is_active: true,
        }
    }
}
