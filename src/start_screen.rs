use std::collections::HashMap;

use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment}, Context};

pub fn render_start_screen(assets : &HashMap<String, Image>, canvas : &mut Canvas, ctx : &mut Context, scl : f32) {
    let (w,h) = ctx.gfx.drawable_size();
    let center = (w / 2., h/2.);

    let image = assets.get("start_screen").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    let press_to_start = Text::new(TextFragment{
        text : "Press Space to start".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(40.0))

    });

    canvas.draw(&press_to_start,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 100., center.1))
            .scale(glam::Vec2::new(scl, scl))
     );

}
