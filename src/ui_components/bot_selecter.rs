use ggez::{
    glam,
    graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment},
    Context,
};
use std::collections::HashMap;

use crate::animation_state::AnimationState;

pub fn render_bot_selector(
    assets: &HashMap<String, Image>,
    canvas: &mut Canvas,
    ctx: &mut Context,
    scl: f32,
    animation_state: &mut AnimationState,
) {
    let (w, h) = ctx.gfx.drawable_size();
    let center = (w / 2., h / 2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(
                center.0 - image_half_size.0,
                center.1 - image_half_size.1,
            ))
            .scale(glam::Vec2::new(scl, scl)),
    );

    let select_bot = Text::new(TextFragment {
        text: "   BOTS".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(80.0)),
    });

    canvas.draw(
        &select_bot,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(
                center.0 - image_half_size.0 + 110.,
                center.1 - 400.,
            ))
            .scale(glam::Vec2::new(scl, scl)),
    );

    let easy = Text::new(TextFragment {
        text: "EASY".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(60.0)),
    });

    canvas.draw(
        &easy,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(
                center.0 - image_half_size.0 + 200.,
                center.1 - 170.,
            ))
            .scale(glam::Vec2::new(scl, scl)),
    );

    let medium = Text::new(TextFragment {
        text: "MEDIUM".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(60.0)),
    });

    canvas.draw(
        &medium,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(
                center.0 - image_half_size.0 + 200.,
                center.1,
            ))
            .scale(glam::Vec2::new(scl, scl)),
    );

    let hard = Text::new(TextFragment {
        text: "HARD".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(60.0)),
    });

    canvas.draw(
        &hard,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(
                center.0 - image_half_size.0 + 200.,
                center.1 + 170.,
            ))
            .scale(glam::Vec2::new(scl, scl)),
    );

    let arrow = Text::new(TextFragment {
        text: ">".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(60.0)),
    });

    let arrow_y = match animation_state.selected_item_bot_selector {
        0 => center.1 - 170.,
        1 => center.1,
        _ => center.1 + 170.,
    };

    canvas.draw(
        &arrow,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(
                center.0 - image_half_size.0 + 100.,
                arrow_y,
            ))
            .scale(glam::Vec2::new(scl, scl)),
    );
}
