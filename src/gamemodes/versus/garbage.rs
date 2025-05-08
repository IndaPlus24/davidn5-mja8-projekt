use std::time::Instant;
use rand::random_range;

use crate::{board::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS}, consts::{GARBAGE_CAP, GARBAGE_DELAY}, Game, PieceType};

impl Game {
    pub fn send_garbage(&mut self, mut amount: usize) {
        self.garbage_sent += amount;

        amount = self.negate_garbage(amount);

        if amount > 0 {
            let column: usize = random_range(0..10);
            let garbage = (column, amount, Some(Instant::now()));
            self.garbage_outbound.push_back(garbage);
        }
    }

    pub fn receive_garbage(&mut self, garbage: (usize, usize, Option<Instant>)) {
        self.garbage_inbound.push_back(garbage);
    }

    pub fn recieve_ready_garbage(&mut self) {
        let mut recieve_count = 0;

        while recieve_count < GARBAGE_CAP {
            if let Some(c) = self.get_garbage_hole() {
                recieve_count += 1;
                self.add_garbage_row(c);

            } else {
                break;
            }
        }
    }

    // Remove garbage from inbound and return rest if present
    fn negate_garbage(&mut self, mut amount: usize) -> usize {
        while self.decrement_garbage() && amount > 0 {amount -= 1}
        amount
    }

    fn decrement_garbage(&mut self) -> bool {
        if let Some(g) = self.garbage_inbound.get_mut(0) {
            g.1 -= 1; // Index of amount of garbage rows
            if g.1 == 0 {self.garbage_inbound.pop_front();}
            return true
        }
        false
    }

    fn get_garbage_hole(&mut self) -> Option<usize> {
        if let Some(g) = self.garbage_inbound.get_mut(0) {
            if g.2 == None {
                g.1 -= 1;
                let column = g.0;
                if g.1 == 0 {self.garbage_inbound.pop_front();}
                return Some(column)
            }
        }
        None
    }

    pub fn update_garbage(&mut self) {
        self.garbage_inbound.iter_mut().for_each(|garbage: &mut (usize, usize, Option<Instant>)| {
            if let Some(t) = garbage.2 {
                if t.elapsed().as_millis() >= GARBAGE_DELAY {
                    garbage.2 = None
                }
            }
        });
    }

    pub fn add_garbage_row(&mut self, column: usize) {
        let mut row = [Some(PieceType::X); BOARD_AMOUNT_COLUMNS];
        row[column] = None;

        // Move everything up one cell
        for i in (1..BOARD_AMOUNT_ROWS).rev() {
            self.board[i] = self.board[i - 1].clone();
        }

        if self.on_ground {
            self.active_piece.midpoint.0 += 1;
        }

        // Insert garbage row
        self.board[0] = row;
        self.garbage_received += 1;
    }
}