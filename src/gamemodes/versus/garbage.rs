use std::time::Instant;
use rand::random_range;

use crate::Game;

impl Game {
    pub fn send_garbage(&mut self, amount: usize) {
        self.attack += amount;
        let column: usize = random_range(0..10);

        let garbage = (column, amount, Some(Instant::now()));
        self.garbage_outbound.push_back(garbage);
    }

    pub fn receive_garbage(&mut self, garbage: (usize, usize, Option<Instant>)) {
        self.garbage_inbound.push_back(garbage);
    }
}