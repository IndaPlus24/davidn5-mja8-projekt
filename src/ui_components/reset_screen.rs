use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextAlign, TextFragment, TextLayout}};
use std::collections::HashMap;

use crate::animation_state::AnimationState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_reset_screen(assets: &HashMap<String, Image>, canvas: &mut Canvas, scl: f32, animation_state: &mut AnimationState){
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    // Game over text
    let mut game_over = Text::new(TextFragment{
        text : "GAME OVER".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(90.))
    });
    game_over.set_layout(TextLayout {
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin
    });
    canvas.draw(&game_over,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1 - 400.))
            .scale(glam::Vec2::new(scl, scl))
    );

    // Options
    let retry = Text::new(TextFragment{
        text : "Retry".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(60.))
    });
    canvas.draw(&retry,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 - 175.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let main_menu = Text::new(TextFragment{
        text : "Return to\nmain menu".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(60.))
    });
    canvas.draw(&main_menu,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    // Arrow
    let arrow = Text::new(TextFragment{
        text : ">".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(60.))
    });

    let arrow_y = match animation_state.selected_item_reset_selector {
        0 => center.1 - 175., 
        _ => center.1 + 30.,
    }; 

    canvas.draw(&arrow,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 100., arrow_y))
            .scale(glam::Vec2::new(scl, scl))
    );
}