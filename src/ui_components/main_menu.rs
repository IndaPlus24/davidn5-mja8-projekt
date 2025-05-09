use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment}};
use std::collections::HashMap;

use crate::animation_state::AnimationState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_main_menu(assets: &HashMap<String, Image>, canvas: &mut Canvas, scl: f32, animation_state: &mut AnimationState){
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    let main_menu = Text::new(TextFragment{
        text: "MAIN MENU".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(90.0))

    });

    canvas.draw(&main_menu,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 110., center.1 - 400.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let start_game = Text::new(TextFragment{
        text: "START GAME".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.0))

    });

    canvas.draw(&start_game,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 - 175.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let highscores = Text::new(TextFragment{
        text: "HIGH SCORES".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.0))

    });

    canvas.draw(&highscores,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    let settings = Text::new(TextFragment{
        text: "SETTINGS".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.0))

    });

    canvas.draw(&settings,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 + 175.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let arrow = Text::new(TextFragment{
        text: ">".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.0))

    });

    let arrow_y = match animation_state.selected_item_main_menu {
        0 => center.1 - 175., 
        1 => center.1,
        _ => center.1 + 175.
    }; 

    canvas.draw(&arrow,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 100., arrow_y))
            .scale(glam::Vec2::new(scl, scl))
    );
}