use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment}};
use std::collections::HashMap;

use crate::animation_state::AnimationState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_gamemode_selector(assets: &HashMap<String, Image>, canvas: &mut Canvas, scl: f32, animation_state: &mut AnimationState){
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    let solo = Text::new(TextFragment{
        text: "   SOLO".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(80.0))

    });

    canvas.draw(&solo,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 110., center.1 - 400.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let marathon = Text::new(TextFragment{
        text: "Marathon".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.0))

    });

    canvas.draw(&marathon,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 - 225.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let fourty_lines = Text::new(TextFragment{
        text: "40 Lines".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.0))

    });

    canvas.draw(&fourty_lines,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 - 75.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let survival = Text::new(TextFragment{
        text: "Survival".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.0))

    });

    canvas.draw(&survival,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 + 75.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let back = Text::new(TextFragment{
        text: "Back".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.0))

    });

    canvas.draw(&back,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 + 225.))
            .scale(glam::Vec2::new(scl, scl))
    );

    
    let arrow = Text::new(TextFragment{
        text: ">".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.0))

    });

    let arrow_y = match animation_state.selected_item_singleplayer_selector {
        0 => {center.1 - 225.},
        1 => {center.1 - 75.},
        2 => {center.1 + 75.},
        _ => {center.1 + 225.}
    };

    canvas.draw(&arrow,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 100., arrow_y))
            .scale(glam::Vec2::new(scl, scl))
    );
}