use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextAlign, TextFragment, TextLayout}};
use std::{collections::HashMap, time::Instant};

use crate::animation_state::AnimationState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_versus_rematch(
    assets: &HashMap<String, Image>,
    canvas: &mut Canvas,
    scl: f32,
    animation_state: &mut AnimationState,
    winner: usize,
    timer: Option<Instant>
) {
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    // Winner
    let mut winner_text = Text::new(TextFragment{
        text: format!("{} wins!", if winner == 0 {"P1"} else {"P2"}),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(100.))
    });
    winner_text.set_layout(TextLayout{
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin
    });
    canvas.draw(&winner_text,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1 - image_half_size.1 + 120.))
            .scale(glam::Vec2::new(scl, scl))
    );

    // Rematch?
    let mut winner_text = Text::new(TextFragment{
        text: "Rematch?".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(80.))
    });
    winner_text.set_layout(TextLayout{
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin
    });
    canvas.draw(&winner_text,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1 - image_half_size.1 + 280.))
            .scale(glam::Vec2::new(scl, scl))
    );


    // P1 rematch stuff
    let mut p1 = Text::new(TextFragment{
        text: "P1".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(80.))
    });
    p1.set_layout(TextLayout{
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin
    });
    canvas.draw(&p1,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - 250. * scl, center.1 - 100. * scl))
            .scale(glam::Vec2::new(scl, scl))
    );

    let checkbox = if animation_state.players_ready.0 {
        assets.get("checkbox_c").unwrap()
    } else {
        assets.get("checkbox_e").unwrap()
    };
    canvas.draw(
        checkbox,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - 414. * scl, center.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    // P2 rematch stuff
    let mut p2 = Text::new(TextFragment{
        text: "P2".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(80.))
    });
    p2.set_layout(TextLayout{
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin
    });
    canvas.draw(&p2,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 + 250. * scl, center.1 - 100. * scl))
            .scale(glam::Vec2::new(scl, scl))
    );

    let checkbox = if animation_state.players_ready.1 {
        assets.get("checkbox_c").unwrap()
    } else {
        assets.get("checkbox_e").unwrap()
    };
    canvas.draw(
        checkbox,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 + 86. * scl, center.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    // Timer
    let time_left = 10 - timer.unwrap().elapsed().as_secs();
    let mut time = Text::new(TextFragment{
        text: format!("Returning to main menu in... {}", time_left),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(25.))
    });
    time.set_layout(TextLayout{
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin
    });
    canvas.draw(&time,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1 + 370.))
            .scale(glam::Vec2::new(scl, scl))
    );
}