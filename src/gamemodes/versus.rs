use std::time::Duration;

use crate::{scoring::ScoreType, Game};

use ggez::{glam, graphics::{self, Canvas, Color, PxScale, Text, TextAlign, TextFragment, TextLayout}};

const ATTACK_VALUES_SINGLE: [usize; 21] =
[0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3];
const ATTACK_VALUES_DOUBLE: [usize; 21] =
[1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6];
const ATTACK_VALUES_TRIPLE: [usize; 21] =
[2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12];
const ATTACK_VALUES_TETRIS: [usize; 21] =
[4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24];
const ATTACK_VALUES_TST: [usize; 21] =
[6, 7, 9, 10, 12, 13, 15, 16, 18, 19, 21, 22, 24, 25, 27, 28, 30, 31, 33, 34, 36];

const ATTACK_VALUES_B2B_TETRIS:  [usize; 21] =
[5, 6, 7, 8, 10, 11, 12, 13, 15, 16, 17, 18, 20, 21, 22, 23, 25, 26, 27, 28, 30];
const ATTACK_VALUES_B2B_TSS: [usize; 21] =
[3, 3, 4, 5, 6, 6, 7, 8, 9, 9, 10, 11, 12, 12, 13, 14, 15, 15, 16, 17, 18];
const ATTACK_VALUES_B2B_TST: [usize; 21] =
[7, 8, 10, 12, 14, 15, 17, 19, 21, 22, 24, 26, 28, 29, 31, 33, 35, 36, 38, 40, 42];

pub fn get_attack_value(score_type: &Option<ScoreType>, b2b: bool, combo: usize) -> usize {
    let mut attack: usize = 0;
    let c = if combo > 20 {20} else {combo};
    
    if let Some(attack_type) = score_type {
        if !b2b {
            attack += match attack_type {
                ScoreType::TSpinMini |
                ScoreType::TSpin => 0,
        
                ScoreType::TSpinMiniSingle |
                ScoreType::Single => ATTACK_VALUES_SINGLE[c],
        
                ScoreType::Double => ATTACK_VALUES_DOUBLE[c],
        
                ScoreType::TSpinSingle |
                ScoreType::Triple => ATTACK_VALUES_TRIPLE[c],
        
                ScoreType::TspinDouble |
                ScoreType::Tetris => ATTACK_VALUES_TETRIS[c],
                
                ScoreType::TSpinTriple => ATTACK_VALUES_TST[c],
            }
        }
        else {
            attack += match attack_type {
                ScoreType::TspinDouble |
                ScoreType::Tetris => ATTACK_VALUES_B2B_TETRIS[c],
    
                ScoreType::TSpinMiniSingle => ATTACK_VALUES_DOUBLE[c],
                ScoreType::TSpinSingle => ATTACK_VALUES_B2B_TSS[c],
                ScoreType::TSpinTriple => ATTACK_VALUES_B2B_TST[c],
    
                _ => 0
            }
        }
    }

    attack
}

impl Game {
    pub fn render_vs_stats(&mut self, canvas: &mut Canvas, pos: (f32, f32), scl: f32) {
        let elapsed = if self.game_over {
            self.final_time
        } else {
            self.start_time.elapsed()
        };

        // Pieces
        let mut pieces = Text::new(TextFragment{
            text: "Pieces".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(16.))
        });
        pieces.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&pieces,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 432. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        let mut piece_count = Text::new(TextFragment{
            text: self.pieces.to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        piece_count.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&piece_count,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 464. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        let formatted_pps = get_formatted_pps(self.pieces, elapsed);
        let mut pps = Text::new(TextFragment{
            text: formatted_pps,
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        pps.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&pps,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 496. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        // APM
        let mut attack = Text::new(TextFragment{
            text: "Attack".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(16.))
        });
        attack.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&attack,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 560. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        let mut sent = Text::new(TextFragment{
            text: self.attack.to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        sent.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&sent,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 592. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        let formatted_apm = get_formatted_apm(self.attack, elapsed);
        let mut apm = Text::new(TextFragment{
            text: formatted_apm,
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        apm.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&apm,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 624. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );
    }
}

fn get_formatted_pps(pieces: usize, duration: Duration) -> String {
    let pps = pieces as f32 / duration.as_secs_f32();
    format!("{:.2}/s", pps)
}
fn get_formatted_apm(attack: usize, duration: Duration) -> String {
    let apm = 60. * attack as f32 / duration.as_secs_f32();
    format!("{:.2}/min", apm)
}