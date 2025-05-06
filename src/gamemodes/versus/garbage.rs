use std::time::Instant;
use rand::random_range;

use crate::Game;

impl Game {
    pub fn send_garbage(&mut self, amount: usize) {
        self.attack += amount;
        let column: usize = random_range(0..10);

        let burst = (column, amount, Some(Instant::now()));
        self.garbage_outbound.push_back(burst);
    }
}