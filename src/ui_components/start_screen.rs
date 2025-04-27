use std::collections::HashMap;

use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment}};

use crate::animation_state::AnimationState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_start_screen(assets : &HashMap<String, Image>, canvas : &mut Canvas, scl : f32, animation_state : &mut AnimationState) {
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

    let image = assets.get("start_screen").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    let press_to_start = Text::new(TextFragment{
        text : "Press Space to\n     start".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(60.0))

    });

    canvas.draw(&press_to_start,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 100., center.1 + animation_state.start_screen_y + 80.))
            .scale(glam::Vec2::new(scl, scl))
     );

     let max_offset = 30.0;
     let spring = -0.001; 
     
     let force = spring * animation_state.start_screen_y;
     animation_state.velocity += force;
     animation_state.start_screen_y += animation_state.velocity;
     
     animation_state.start_screen_y = animation_state.start_screen_y.clamp(-max_offset, max_offset);
}
