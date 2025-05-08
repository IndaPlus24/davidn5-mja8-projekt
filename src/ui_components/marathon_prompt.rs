use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextAlign, TextFragment, TextLayout}};
use std::collections::HashMap;

use crate::animation_state::AnimationState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_marathon_prompt(assets: &HashMap<String, Image>, canvas: &mut Canvas, scl: f32, animation_state: &mut AnimationState){
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    // Marathon
    let mut game_over = Text::new(TextFragment{
        text: "Marathon".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(90.))
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

    // Start Level
    let mut start_level = Text::new(TextFragment{
        text: "Starting Level".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.))
    });
    start_level.set_layout(TextLayout {
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin
    });
    canvas.draw(&start_level,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1 - 100.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let mut level = Text::new(TextFragment{
        text: animation_state.selected_item_marathon_prompt.0.to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.))
    });
    level.set_layout(TextLayout {
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin
    });
    canvas.draw(&level,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    // Confirm
    let mut confirm = Text::new(TextFragment{
        text: "CONFIRM".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(60.))
    });
    confirm.set_layout(TextLayout {
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin
    });
    canvas.draw(&confirm,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1 + 200.))
            .scale(glam::Vec2::new(scl, scl))
    );

    // Arrow(s)
    if animation_state.selected_item_marathon_prompt.1 == 0 {
        let one_wide = animation_state.selected_item_marathon_prompt.0 < 10;
        let mut arrows = Text::new(TextFragment{
            text: (if one_wide {"< >"} else {"<  >"}).to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(60.))
        });
        arrows.set_layout(TextLayout {
            h_align: TextAlign::Middle,
            v_align: TextAlign::Begin
        });
        canvas.draw(&arrows,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0, center.1))
                .scale(glam::Vec2::new(scl, scl))
        );
    } else {
        let arrow = Text::new(TextFragment{
            text: ">".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(60.))
        });
        canvas.draw(&arrow,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - 300., center.1 + 200.))
                .scale(glam::Vec2::new(scl, scl))
        );
    }
}