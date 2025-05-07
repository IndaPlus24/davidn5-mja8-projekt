use ggez::{glam, graphics::{self, Canvas, Color, PxScale, Text, TextAlign, TextFragment, TextLayout}};

use crate::{ui_components::stat_formatting::*, Game};

impl Game {
    pub fn render_survival_stats(&mut self, canvas: &mut Canvas, pos: (f32, f32), scl: f32) {
        let elapsed = if self.game_over {
            self.final_time
        } else {
            self.start_time.elapsed()
        };

        // Level
        let mut level = Text::new(TextFragment{
            text: "Level".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(16.))
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

        let mut received_count = Text::new(TextFragment{
            text: self.garbage_received.to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(24.))
        });
        received_count.set_layout(TextLayout{
            h_align: TextAlign::End,
            v_align: TextAlign::Middle
        });
        canvas.draw(&received_count,
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
