use std::time::Instant;
use rand::random_range;

use crate::Game;

impl Game {
    pub fn send_garbage(&mut self, mut amount: usize) {
        self.attack += amount;

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