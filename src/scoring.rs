use crate::Game;

pub enum ScoreType {
    Single,
    Double,
    Triple,
    Tetris,
    TSpinMini,
    TSpin,
    TSpinMiniSingle,
    TSpinSingle,
    TspinDouble,
    TSpinTriple,
}

impl Game {
    pub fn add_score(&mut self, score_type: Option<ScoreType>) {
        let mut points = 0.;
        if let Some(s) = score_type {
            match s {
                ScoreType::Single          => {points += 100.; println!("Single!")},
                ScoreType::Double          => {points += 300.; println!("Double!")},
                ScoreType::Triple          => {points += 500.; println!("Triple!")},
                ScoreType::Tetris          => {points += 800.; println!("Tetris!")},
                ScoreType::TSpinMini       => {points += 100.; println!("T-Spin Mini!")},
                ScoreType::TSpin           => {points += 400.; println!("T-Spin!")},
                ScoreType::TSpinMiniSingle => {points += 200.; println!("T-Spin Mini Single!")},
                ScoreType::TSpinSingle     => {points += 800.; println!("T-Spin Single!")},
                ScoreType::TspinDouble     => {points += 1200.; println!("T-Spin Double!")},
                ScoreType::TSpinTriple     => {points += 1600.; println!("T-Spin Triple!")},
            };

            // Combo check
            match s {
                // No line clear breaks combo
                ScoreType::TSpinMini |
                ScoreType::TSpin => {
                    self.prev_clear = false;
                    self.combo = 0;
                },
                // Line clear
                _ => {
                    if self.prev_clear {
                        self.combo += 1;
                        points += 50. * self.combo as f32;
                        println!("Combo {}!", self.combo);
                    }
                    self.prev_clear = true;
                }
            }

            // Back-to-Back check
            match s {
                ScoreType::Single |
                ScoreType::Double |
                ScoreType::Triple => {
                    self.latest_clear_difficult = false;
                    self.back_to_back = false;
                }
                ScoreType::TSpinMini |
                ScoreType::TSpin => (),
                _ => {
                    if self.back_to_back {
                        points *= 1.5;
                    }
                    self.back_to_back = true;
                }
            }

            // Perfect clear check
            if self.board[0].iter().all(|c| c.is_none()) {
                println!("Perfect clear!");
                points += match s {
                    ScoreType::Single => 800.,
                    ScoreType::Double => 1200.,
                    ScoreType::Triple => 1800.,
                    ScoreType::Tetris => {
                        if self.back_to_back {3200.}
                        else {2000.}
                    },
                    _ => 0.
                }
            }
            
            match s {
                ScoreType::TSpinMini |
                ScoreType::TSpin => self.prev_clear = false,
                _ => self.prev_clear = true,
            }
        }
        else {
            self.prev_clear = false;
            self.combo = 0;
        }

        points *= self.level as f32;

        self.score += points as usize;
    }
}