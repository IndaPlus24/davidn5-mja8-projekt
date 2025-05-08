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

    let labels = ["Arr", "Das", "Sds"];
    let values = [
        state.game_one.arr.as_millis(),
        state.game_one.das.as_millis(),
        state.game_one.sds as u128,
    ];

    let start_y = center.1 - 150.;
    let spacing = 100.;

    for (i, (label, value)) in labels.iter().zip(values.iter()).enumerate() {
        let y = start_y + i as f32 * spacing;

        let mut text_label = Text::new(TextFragment {
            text: label.to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(60.)),
        });
        text_label.set_layout(TextLayout {
            h_align: TextAlign::Middle,
            v_align: TextAlign::Begin,
        });
        canvas.draw(
            &text_label,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - 250., y))
                .scale(glam::Vec2::new(scl, scl)),
        );

        let mut text_value = Text::new(TextFragment {
            text: value.to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(60.)),
        });
        text_value.set_layout(TextLayout {
            h_align: TextAlign::Middle,
            v_align: TextAlign::Begin,
        });
        canvas.draw(
            &text_value,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0, y))
                .scale(glam::Vec2::new(scl, scl)),
        );

        if state.animation_state.selected_item_settings == i {
            let one_wide = *value < 10;
            let three_wide = *value > 99;
            let arrows_text = if one_wide {
                "< >"
            } else if three_wide {
                "<   >"
            } else {
                "<  >"
            };

            let mut arrows = Text::new(TextFragment {
                text: arrows_text.to_string(),
                font: Some("Tetris font".to_string()),
                color: Some(Color::WHITE),
                scale: Some(PxScale::from(60.)),
            });
            arrows.set_layout(TextLayout {
                h_align: TextAlign::Middle,
                v_align: TextAlign::Begin,
            });
            canvas.draw(
                &arrows,
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(center.0, y))
                    .scale(glam::Vec2::new(scl, scl)),
            );
        }
    }

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
    let confirm_y = start_y + labels.len() as f32 * spacing + 100.;
    canvas.draw(
        &confirm,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0, confirm_y))
            .scale(glam::Vec2::new(scl, scl)),
    );

    if state.animation_state.selected_item_settings == labels.len() {
        let arrow = Text::new(TextFragment {
            text: ">".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE),
            scale: Some(PxScale::from(60.)),
        });
        canvas.draw(
            &arrow,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - 300., confirm_y))
                .scale(glam::Vec2::new(scl, scl)),
        );
    }
}
