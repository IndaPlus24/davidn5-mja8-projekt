use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment}, Context};
use std::collections::HashMap;

use crate::animation_state::AnimationState;

pub fn render_main_menu(assets : &HashMap<String, Image>, canvas : &mut Canvas, ctx : &mut Context, scl : f32){
    let (w,h) = ctx.gfx.drawable_size();
    let center = (w / 2., h/2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );
}