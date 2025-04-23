use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment}, Context};
use std::collections::HashMap;

use crate::animation_state::{self, AnimationState};

pub fn render_main_menu(assets : &HashMap<String, Image>, canvas : &mut Canvas, ctx : &mut Context, scl : f32, animation_state : &mut AnimationState){
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

    let main_menu = Text::new(TextFragment{
        text : "MAIN MENU".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(90.0))

    });

    canvas.draw(&main_menu,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 110., center.1 - 400.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let start_game = Text::new(TextFragment{
        text : "START GAME".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(60.0))

    });

    canvas.draw(&start_game,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 - 100.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let settings = Text::new(TextFragment{
        text : "SETTINGS".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(60.0))

    });

    canvas.draw(&settings,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 + 100.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let arrow = Text::new(TextFragment{
        text : ">".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(60.0))

    });

    let arrow_y = if animation_state.selected_item_main_menu == 0 {center.1 - 100.}else {center.1 + 100.};

    canvas.draw(&arrow,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 100., arrow_y))
            .scale(glam::Vec2::new(scl, scl))
    );


}