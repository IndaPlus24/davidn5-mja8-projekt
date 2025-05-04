use ggez::{
    glam,
    graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment},
};
use std::collections::HashMap;

use crate::animation_state::AnimationState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_bot_selector(
    assets: &HashMap<String, Image>,
    canvas: &mut Canvas,
    scl: f32,
    animation_state: &mut AnimationState,
) {
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

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
                center.1 - 225.,
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
                center.1 - 75.,
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
                center.1 + 75.,
            ))
            .scale(glam::Vec2::new(scl, scl)),
    );

    let back = Text::new(TextFragment {
        text: "Back".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(60.0)),
    });

    canvas.draw(
        &back,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(
                center.0 - image_half_size.0 + 200.,
                center.1 + 225.,
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
        0 => center.1 - 225.,
        1 => center.1 - 75.,
        2 => center.1 + 75.,
        _ => center.1 + 225.,
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
