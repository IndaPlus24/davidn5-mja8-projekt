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
    pub fn add_score(&mut self, score_type: &Option<ScoreType>) {
        let mut points = 0.;
        if let Some(s) = score_type {
            points += match s {
                ScoreType::Single          =>  100.,
                ScoreType::Double          =>  300.,
                ScoreType::Triple          =>  500.,
                ScoreType::Tetris          =>  800.,
                ScoreType::TSpinMini       =>  100.,
                ScoreType::TSpin           =>  400.,
                ScoreType::TSpinMiniSingle =>  200.,
                ScoreType::TSpinSingle     =>  800.,
                ScoreType::TspinDouble     => 1200.,
                ScoreType::TSpinTriple     => 1600.,
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
                    
                    if self.latest_clear_difficult {self.back_to_back = true}
                    self.latest_clear_difficult = true;
                }
            }

            // Perfect clear check
            if self.board[0].iter().all(|c| c.is_none()) {
                points += match s {
                    ScoreType::Single => 800.,
                    ScoreType::Double => 1200.,
                    ScoreType::Triple => 1800.,
                    ScoreType::Tetris => {
                        if self.back_to_back {3200.}
                        else {2000.}
                    },
                    _ => 0.
                };

                self.all_clear = true;
            }
            else {self.all_clear = false}
            
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