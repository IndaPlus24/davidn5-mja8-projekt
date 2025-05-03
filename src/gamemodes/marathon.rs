use ggez::{glam, graphics::{self, Canvas, Color, PxScale, Text, TextAlign, TextFragment, TextLayout}};

use crate::Game;

impl Game {
    pub fn render_marathon_stats(&mut self, canvas: &mut Canvas, pos: (f32, f32), scl: f32) {
        // Score
        let formatted_score = get_formatted_score(self.score);
        let mut score = Text::new(TextFragment{
            text: formatted_score,
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        score.set_layout(TextLayout::center());
        canvas.draw(&score,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 328. * scl, pos.1 + 668. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        // Level
        let mut level = Text::new(TextFragment{
            text: "Level".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        level.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&level,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 496. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        let mut level_count = Text::new(TextFragment{
            text: self.level.to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(16.))
        });
        level_count.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&level_count,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 528. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        // Lines
        let mut lines = Text::new(TextFragment{
            text: "Lines".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        lines.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&lines,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 592. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        let mut line_count = Text::new(TextFragment{
            text: format!("{}/150", self.lines),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(16.))
        });
        line_count.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&line_count,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 624. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );
    }
}

fn get_formatted_score(score: usize) -> String {
    let mut s = score;
    let mega = s / 1_000_000;
    s %= 1_000_000;
    let kilo = s / 1_000;
    s %= 1_000;

    if mega == 0 {
        if kilo == 0 {
            return s.to_string();
        }
        return format!("{},{:0>3}", kilo, s);
    }
    return format!("{},{:0>3},{:0>3}", mega, kilo, s);
}