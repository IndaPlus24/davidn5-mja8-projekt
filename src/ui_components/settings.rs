use ggez::{
    glam,
    graphics::{self, Canvas, Color, PxScale, Text, TextAlign, TextFragment, TextLayout},
};

use crate::AppState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_settings(canvas: &mut Canvas, scl: f32, state: &mut AppState) {
    let assets = &state.menu_assets;
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0, center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl)),
    );
    
    // Title
    let mut title = Text::new(TextFragment {
        text: "Settings".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(90.)),
    });
    title.set_layout(TextLayout {
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin,
    });
    canvas.draw(
        &title,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1 - 400.))
            .scale(glam::Vec2::new(scl, scl)),
    );

    // Instructions
    let mut title = Text::new(TextFragment {
        text: "Hold SELECT to edit value".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(30.)),
    });
    title.set_layout(TextLayout {
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin,
    });
    canvas.draw(
        &title,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1 - 280.))
            .scale(glam::Vec2::new(scl, scl)),
    );

    // P1 settings
    let mut title = Text::new(TextFragment {
        text: "P1".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(80.)),
    });
    title.set_layout(TextLayout {
        h_align: TextAlign::Middle,
        v_align: TextAlign::End,
    });
    canvas.draw(
        &title,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - 230., center.1 - 50.))
            .scale(glam::Vec2::new(scl, scl)),
    );

    let values = [
        state.game_one.das.as_millis(),
        state.game_one.arr.as_millis(),
        state.game_one.sds as u128,
    ];
    for (i, value) in values.iter().enumerate() {
        let mut selected = true;
        selected &= state.animation_state.selected_item_settings.0 == 0;
        selected &= state.animation_state.selected_item_settings.1 == i;
        let editing = state.animation_state.edit_setting_value;

        let y = center.1 + i as f32 * 75.;

        // Label
        let mut text_label = Text::new(TextFragment {
            text: match i {
                0 => "DAS:".to_string(),
                1 => "ARR:".to_string(),
                _ => "SDS:".to_string(),
            },
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(40.)),
        });
        text_label.set_layout(TextLayout {
            h_align: TextAlign::Middle,
            v_align: TextAlign::Begin,
        });
        canvas.draw(
            &text_label,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - 320., y))
                .scale(glam::Vec2::new(scl, scl)),
        );

        // Value
        let mut value_text = Text::new(TextFragment {
            text: if *value > 500 {"inf".to_string()} else {value.to_string()},
            font: Some("Tetris font".to_string()),
            color: Some(if selected && editing {Color::YELLOW} else {Color::WHITE}),
            scale: Some(PxScale::from(40.)),
        });
        value_text.set_layout(TextLayout {
            h_align: TextAlign::Middle,
            v_align: TextAlign::Begin,
        });
        canvas.draw(
            &value_text,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - 140., y))
                .scale(glam::Vec2::new(scl, scl)),
        );

        // Arrows
        if selected {
            let arrows_text =
            if *value < 10 {"< >"}
            else if *value < 100 {"<  >"}
            else {"<   >"};

            let mut arrows = Text::new(TextFragment {
                text: arrows_text.to_string(),
                font: Some("Tetris font".to_string()),
                color: Some(if selected && editing {Color::YELLOW} else {Color::WHITE}),
                scale: Some(PxScale::from(40.)),
            });
            arrows.set_layout(TextLayout {
                h_align: TextAlign::Middle,
                v_align: TextAlign::Begin,
            });
            canvas.draw(
                &arrows,
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(center.0 - 140., y))
                    .scale(glam::Vec2::new(scl, scl)),
            );
        }
    }

    // P2 settings
    let mut title = Text::new(TextFragment {
        text: "P2".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(80.)),
    });
    title.set_layout(TextLayout {
        h_align: TextAlign::Middle,
        v_align: TextAlign::End,
    });
    canvas.draw(
        &title,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 + 230., center.1 - 50.))
            .scale(glam::Vec2::new(scl, scl)),
    );

    let values = [
        state.game_two.das.as_millis(),
        state.game_two.arr.as_millis(),
        state.game_two.sds as u128,
    ];
    for (i, value) in values.iter().enumerate() {
        let mut selected = true;
        selected &= state.animation_state.selected_item_settings.0 == 1;
        selected &= state.animation_state.selected_item_settings.1 == i;
        let editing = state.animation_state.edit_setting_value;

        let y = center.1 + i as f32 * 75.;

        // Label
        let mut text_label = Text::new(TextFragment {
            text: match i {
                0 => "DAS:".to_string(),
                1 => "ARR:".to_string(),
                _ => "SDS:".to_string(),
            },
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(40.)),
        });
        text_label.set_layout(TextLayout {
            h_align: TextAlign::Middle,
            v_align: TextAlign::Begin,
        });
        canvas.draw(
            &text_label,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 + 140., y))
                .scale(glam::Vec2::new(scl, scl)),
        );

        // Value
        let mut value_text = Text::new(TextFragment {
            text: if *value > 500 {"inf".to_string()} else {value.to_string()},
            font: Some("Tetris font".to_string()),
            color: Some(if selected && editing {Color::YELLOW} else {Color::WHITE}),
            scale: Some(PxScale::from(40.)),
        });
        value_text.set_layout(TextLayout {
            h_align: TextAlign::Middle,
            v_align: TextAlign::Begin,
        });
        canvas.draw(
            &value_text,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 + 320., y))
                .scale(glam::Vec2::new(scl, scl)),
        );

        // Arrows
        if selected {
            let arrows_text =
            if *value < 10 {"< >"}
            else if *value < 100 {"<  >"}
            else {"<   >"};

            let mut arrows = Text::new(TextFragment {
                text: arrows_text.to_string(),
                font: Some("Tetris font".to_string()),
                color: Some(if selected && editing {Color::YELLOW} else {Color::WHITE}),
                scale: Some(PxScale::from(40.)),
            });
            arrows.set_layout(TextLayout {
                h_align: TextAlign::Middle,
                v_align: TextAlign::Begin,
            });
            canvas.draw(
                &arrows,
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(center.0 + 320., y))
                    .scale(glam::Vec2::new(scl, scl)),
            );
        }
    }

    // Confirm
    let mut confirm = Text::new(TextFragment {
        text: "CONFIRM".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(60.)),
    });
    confirm.set_layout(TextLayout {
        h_align: TextAlign::Middle,
        v_align: TextAlign::Begin,
    });
    canvas.draw(
        &confirm,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, center.1 + 300.))
            .scale(glam::Vec2::new(scl, scl)),
    );

    if state.animation_state.selected_item_settings.1 == 3 {
        let arrow = Text::new(TextFragment {
            text: ">".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(60.)),
        });
        canvas.draw(
            &arrow,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - 300., center.1 + 300.))
                .scale(glam::Vec2::new(scl, scl)),
        );
    }
}
