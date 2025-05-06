use ggez::{glam, graphics::{self, Canvas, Color, PxScale, Text, TextFragment}};

use crate::AppState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn render_input_name(state : &mut AppState, canvas: &mut Canvas, scl: f32){
    let animation_state = &mut state.animation_state;
    let assets = &state.menu_assets;
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    let titel = Text::new(TextFragment{
        text : "INPUT NAME".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(80.0))

    });

    canvas.draw(&titel,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 110., center.1 - 400.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let name_display = Text::new(TextFragment {
        text: animation_state.name_input.clone(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE),
        scale: Some(PxScale::from(60.0)),
    });
    
    canvas.draw(&name_display, graphics::DrawParam::new()
        .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 - 150.))
        .scale(glam::Vec2::new(scl, scl))
    );
    

    let keyboard = vec![
    vec!["Q","W","E","R","T","Y","U","I","O","P","Å"],
    vec!["A","S","D","F","G","H","I","J","K","L","Ö","Ä"],
    vec!["Z","X","C","V","B","N","M","<"]];

    let mut sizes = Vec::new();

    for i in 30..=50 {
        sizes.push(i as f32);
    }
    
    for i in (30..50).rev() {
        sizes.push(i as f32);
    }

    animation_state.ticks += 1;


    for (i,row) in keyboard.iter().enumerate() {
        for (k, key) in row.iter().enumerate() {
            let mut size = 30.0;
            if animation_state.selected_item_high_score.1 == 0 && animation_state.selected_key == (i, k) {
                animation_state.ticks = 0;
                size = sizes[animation_state.size_index];
                animation_state.size_index = (animation_state.size_index + 1) % sizes.len();
            }
            
            if (i,k) == (2,7){
                let cont = Text::new(TextFragment{
                    text : format!("<-"),
                    font : Some("Tetris font".to_string()),
                    color: Some(Color::WHITE), 
                    scale : Some(PxScale::from(size))
            
                });
            
                canvas.draw(&cont,
                    graphics::DrawParam::new()
                        .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200.+ 50. * k as f32, center.1 + i as f32 * 50.))
                        .scale(glam::Vec2::new(scl, scl))
                );
            }else {
                let cont = Text::new(TextFragment{
                    text : format!("{}", key),
                    font : Some("Tetris font".to_string()),
                    color: Some(Color::WHITE), 
                    scale : Some(PxScale::from(size))
            
                });
            
                canvas.draw(&cont,
                    graphics::DrawParam::new()
                        .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200.+ 50. * k as f32, center.1 + i as f32 * 50.))
                        .scale(glam::Vec2::new(scl, scl))
                );
            }
        }
    }
   

    let cont = Text::new(TextFragment{
        text : " CONTINUE".to_string(),
        font : Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale : Some(PxScale::from(60.0))

    });

    canvas.draw(&cont,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 + 225.))
            .scale(glam::Vec2::new(scl, scl))
    );

    if animation_state.selected_item_high_score.1 == 1 {
        let arrow = Text::new(TextFragment{
            text : ">".to_string(),
            font : Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale : Some(PxScale::from(60.0))
    
        });
    
        canvas.draw(&arrow,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - image_half_size.0 + 100., center.1 + 225.))
                .scale(glam::Vec2::new(scl, scl))
        );
    }
}