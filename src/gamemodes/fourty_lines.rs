use std::time::Duration;

use ggez::{glam, graphics::{self, Canvas, Color, PxScale, Text, TextAlign, TextFragment, TextLayout}};

use crate::Game;

impl Game {
    pub fn render_40l_stats(&mut self, canvas: &mut Canvas, pos: (f32, f32), scl: f32) {
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
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 368. * scl))
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
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 400. * scl))
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
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 432. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        // Lines
        let mut lines = Text::new(TextFragment{
            text: "Lines".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(16.))
        });
        lines.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&lines,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 496. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        let mut line_count = Text::new(TextFragment{
            text: format!("{}/40", self.lines),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        line_count.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&line_count,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 528. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        // Time
        let mut time = Text::new(TextFragment{
            text: "Time".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(16.))
        });
        time.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&time,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 592. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );

        let formatted_time = get_formatted_time(elapsed);
        let mut duration = Text::new(TextFragment{
            text: formatted_time,
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        duration.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&duration,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + 156. * scl, pos.1 + 624. * scl))
                .scale(glam::Vec2::new(scl, scl))
        );
    }
}

fn get_formatted_time(duration: Duration) -> String {
    let mut secs = duration.as_secs();
    let mins = secs / 60;
    secs %= 60;
    let millis = duration.subsec_millis();
    
    format!("{}:{:0>2}.{:0>3}", mins, secs, millis)
}

fn get_formatted_pps(pieces: usize, duration: Duration) -> String {
    let pps = pieces as f32 / duration.as_secs_f32();
    format!("{:.2}/s", pps)
}