use std::time::Instant;

use crate::{Game, Piece, PieceType};

pub const ROTATION_CW:  usize = 1;
pub const ROTATION_180: usize = 2;
pub const ROTATION_CCW: usize = 3;

pub const RELATIVE_MINOS_I: [[(isize, isize); 4]; 4] = [ // Offsets from piece midpoint (DR, DC)
    [( 0, -1), ( 0, 0), ( 0, 1), (0, 2)], // Rotation index 0: spawn orientation
    [( 1,  1), ( 0, 1), (-1, 1), (-2, 1)], //                1: 90 degrees cw
    [(-1, -1), (-1, 0), (-1, 1), (-1, 2)], //                2: 180 degrees
    [( 1,  0), ( 0, 0), (-1, 0), (-2, 0)], //                3: 90 degrees ccw
];

pub const RELATIVE_MINOS_J: [[(isize, isize); 4]; 4] = [
    [(1, -1), (0, -1), ( 0,  0), ( 0, 1)],
    [(1,  0), (1,  1), ( 0,  0), (-1, 0)],
    [(0, -1), (0,  0), ( 0,  1), (-1, 1)],
    [(1,  0), (0,  0), (-1, -1), (-1, 0)],
];

pub const RELATIVE_MINOS_L: [[(isize, isize); 4]; 4] = [
    [(1,  1), (0, -1), ( 0, 0), ( 0,  1)],
    [(1,  0), (0,  0), (-1, 0), (-1,  1)],
    [(0, -1), (0,  0), ( 0, 1), (-1, -1)],
    [(1, -1), (1,  0), ( 0, 0), (-1,  0)],
];

pub const RELATIVE_MINOS_O: [[(isize, isize); 4]; 4] = [
    [(1, 0), (1, 1), (0, 0), (0, 1)],
    [(1, 0), (1, 1), (0, 0), (0, 1)],
    [(1, 0), (1, 1), (0, 0), (0, 1)],
    [(1, 0), (1, 1), (0, 0), (0, 1)],
];

pub const RELATIVE_MINOS_S: [[(isize, isize); 4]; 4] = [
    [(1,  0), (1,  1), ( 0, -1), ( 0, 0)],
    [(1,  0), (0,  0), ( 0,  1), (-1, 1)],
    [(0,  0), (0,  1), (-1, -1), (-1, 0)],
    [(1, -1), (0, -1), ( 0,  0), (-1, 0)],
];

pub const RELATIVE_MINOS_T: [[(isize, isize); 4]; 4] = [
    [(1,  0), (0, -1), (0, 0), ( 0, 1)],
    [(1,  0), (0,  0), (0, 1), (-1, 0)],
    [(0, -1), (0,  0), (0, 1), (-1, 0)],
    [(1,  0), (0, -1), (0, 0), (-1, 0)],
];

pub const RELATIVE_MINOS_Z: [[(isize, isize); 4]; 4] = [
    [(1, -1), (1,  0), ( 0, 0), ( 0,  1)],
    [(1,  1), (0,  0), ( 0, 1), (-1,  0)],
    [(0, -1), (0,  0), (-1, 0), (-1,  1)],
    [(1,  0), (0, -1), ( 0, 0), (-1, -1)],
];

pub const KICK_TABLE_CW_REGULAR: [[(isize, isize); 6]; 4] = [ // Positions to check for a clockwise rotation of J, L, S, Z or T (dx, dy)
    [(0, 0), (-1, 0), (-1, -1), (0,  2), (-1,  2), (0, 0)], // 3 -> 0
    [(0, 0), (-1, 0), (-1,  1), (0, -2), (-1, -2), (0, 0)], // 0 -> 1
    [(0, 0), ( 1, 0), ( 1, -1), (0,  2), ( 1,  2), (0, 0)], // 1 -> 2
    [(0, 0), ( 1, 0), ( 1,  1), (0, -2), ( 1, -2), (0, 0)], // 2 -> 3
];

pub const KICK_TABLE_CW_I: [[(isize, isize); 6]; 4] = [
    [(0, 0), ( 1, 0), (-2, 0), ( 1, -2), (-2,  1), (0, 0)], // 3 -> 0
    [(0, 0), ( 1, 0), (-2, 0), (-2, -1), ( 1,  2), (0, 0)], // 0 -> 1
    [(0, 0), (-1, 0), ( 2, 0), (-1,  2), ( 2, -1), (0, 0)], // 1 -> 2
    [(0, 0), ( 2, 0), (-1, 0), ( 2,  1), (-1, -2), (0, 0)], // 2 -> 3
];

pub const KICK_TABLE_CCW_REGULAR: [[(isize, isize); 6]; 4] = [
    [(0, 0), ( 1, 0), ( 1, -1), (0,  2), ( 1,  2), (0, 0)], // 1 -> 0
    [(0, 0), (-1, 0), (-1,  1), (0, -2), (-1, -2), (0, 0)], // 2 -> 1
    [(0, 0), (-1, 0), (-1, -1), (0,  2), (-1,  2), (0, 0)], // 3 -> 2
    [(0, 0), ( 1, 0), ( 1,  1), (0, -2), ( 1, -2), (0, 0)], // 0 -> 3
];

pub const KICK_TABLE_CCW_I: [[(isize, isize); 6]; 4] = [
    [(0, 0), (-1, 0), ( 2, 0), (-1, -2), ( 2,  1), (0, 0)], // 1 -> 0
    [(0, 0), (-2, 0), ( 1, 0), (-2,  1), (-1, -2), (0, 0)], // 2 -> 1
    [(0, 0), ( 1, 0), (-2, 0), ( 1,  2), (-2, -1), (0, 0)], // 3 -> 2
    [(0, 0), (-1, 0), ( 2, 0), ( 2, -1), (-1,  2), (0, 0)], // 0 -> 3
];

pub const KICK_TABLE_180: [[(isize, isize); 6]; 4] = [
    [(0, 0), ( 0, -1), (-1, -1), ( 1, -1), (-1, 0), ( 1, 0)], // 2 -> 0
    [(0, 0), (-1,  0), (-1,  2), (-1,  1), ( 0, 2), ( 0, 1)], // 3 -> 1
    [(0, 0), ( 0,  1), ( 1,  1), (-1,  1), ( 1, 0), (-1, 0)], // 0 -> 2
    [(0, 0), ( 1,  0), ( 1,  2), ( 1,  1), ( 0, 2), ( 0, 1)], // 1 -> 3
];

impl Game {
    pub fn rotate(&mut self, rotation_type: usize) -> bool {
        // Save old ground state
        let on_ground = self.on_ground;
        let on_ground_start = self.on_ground_start;

        // Set up rotated piece for kick table checks
        let piece = self.active_piece.clone();
        let new_rotation: usize = (piece.rotation + rotation_type) % 4;
        let mut rotated_piece = Piece::new(piece.piece_type, new_rotation);
        rotated_piece.midpoint = piece.midpoint;
        self.active_piece = rotated_piece;

        // Fetch the suitable kick table
        let kick_table = match rotation_type {
            ROTATION_CW => match piece.piece_type {
                PieceType::I => KICK_TABLE_CW_I[new_rotation],
                _ => KICK_TABLE_CW_REGULAR[new_rotation],
            },
            ROTATION_CCW => match piece.piece_type {
                PieceType::I => KICK_TABLE_CCW_I[new_rotation],
                _ => KICK_TABLE_CCW_REGULAR[new_rotation],
            },
            _ => KICK_TABLE_180[new_rotation],
        };
        
        // Try kick table offsets
        for (dx, dy) in kick_table {
            if self.move_piece(dx, dy) {
                self.active_piece.rotation = new_rotation;

                // Check for T-Spin and T-Spin Mini
                if piece.piece_type == PieceType::T {
                    let (mr, mc) = self.active_piece.midpoint;
                    // Four corners around midpoint (dr, dc)
                    let around: [(isize, isize); 4] = [(1, -1), (1, 1), (-1, 1), (-1, -1)];
                    // Amount of corners in front of the t-piece that is solid (0, 1 or 2)
                    let mut front_count = 0;
                    // Amount of corners behind the t-piece that is solid (0, 1 or 2)
                    let mut back_count = 0;

                    for (i, (dr, dc)) in around.iter().enumerate() {
                        if self.is_solid_tile(mr + dr, mc + dc) {
                            if i == new_rotation || i == (new_rotation + 1) % 4 {
                                front_count += 1;
                            } else {
                                back_count += 1;
                            }
                        }
                    }

                    if front_count + back_count >= 3 {
                        self.t_spin = true;
                        self.t_spin_mini = front_count == 1;
                    }
                }

                if self.on_ground {self.last_drop = Instant::now()}
                self.add_action();
                return true;
            }
        }

        // Failed rotation
        self.on_ground = on_ground;
        self.on_ground_start = on_ground_start;
        self.active_piece = piece;
        false
    }
}