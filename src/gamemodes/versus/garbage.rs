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

    pub fn update_garbage(&mut self) {
        self.garbage_inbound.iter_mut().for_each(|garbage: &mut (usize, usize, Option<Instant>)| {
            if let Some(t) = garbage.2 {
                if t.elapsed().as_millis() >= 500 {
                    garbage.2 = None
                }
            }
        });
    }
}