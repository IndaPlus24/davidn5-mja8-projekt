use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment}};
use std::{collections::HashMap, time::Duration};

use crate::AppState;
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::stat_formatting::{get_formatted_score, get_formatted_time};

pub fn render_high_score(state: &AppState ,assets: &HashMap<String, Image>, canvas: &mut Canvas, scl: f32){
    let animation_state = &state.animation_state;
    let center = (WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.);

    let image = assets.get("empty_box").unwrap();
    let image_half_size = (image.width() as f32 / 2., image.height() as f32 / 2.);

    canvas.draw(
        image,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0,center.1 - image_half_size.1))
            .scale(glam::Vec2::new(scl, scl))
    );

    let high_score = Text::new(TextFragment{
        text: "HIGH SCORE".to_string(),
        font: Some("Tetris font".to_string()),
        color: Some(Color::WHITE), 
        scale: Some(PxScale::from(80.0))

    });

    canvas.draw(&high_score,
        graphics::DrawParam::new()
            .dest(glam::Vec2::new(center.0 - image_half_size.0 + 110., center.1 - 400.))
            .scale(glam::Vec2::new(scl, scl))
    );

    let titles = ["SURVIVAL", "MARATHON", "40L"];
    if animation_state.selected_item_high_score.0 == 0 || animation_state.selected_item_high_score.0 == 2 {

        let labels = Text::new(TextFragment{
            text: "NAME      TIME ".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(20.0))
    
        });
    
        canvas.draw(&labels,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - image_half_size.0 + 300., center.1 - 200.))
                .scale(glam::Vec2::new(scl, scl))
        );
    }else {
        let labels = Text::new(TextFragment{
            text: "NAME      SCORE".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(20.0))
    
        });
    
        canvas.draw(&labels,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - image_half_size.0 + 300., center.1 - 200.))
                .scale(glam::Vec2::new(scl, scl))
        );
    }

    for i in 0..3 {

        
        let mut text = titles[i].to_string(); 

        if animation_state.selected_item_high_score == (i as i32,0){
            text = format!(">{}", text);
        }
        let gamemode = Text::new(TextFragment{
            text: text,
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(30.0))
    
        });
    
        canvas.draw(&gamemode,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - image_half_size.0 + 110. + i as f32 * 300., center.1 - 275.))
                .scale(glam::Vec2::new(scl, scl))
        );
    }

    let scores = &animation_state.highscore_list;

    for p in 0..5 {
        let mut _text: String = "".to_string();
        if animation_state.selected_item_high_score.0 == 0 || animation_state.selected_item_high_score.0 == 2{
            _text = format!("{}. {} : {}",p + 1,scores[p].0,get_formatted_time(Duration::from_millis(scores[p].1 as u64))); 
        }else {
            _text = format!("{}. {} : {}",p + 1,scores[p].0,get_formatted_score(scores[p].1 )); 
        }

        let player1 = Text::new(TextFragment{
            text: _text,
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(30.0))
    
        });
    
        canvas.draw(&player1,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - image_half_size.0 + 200., center.1 - 150. + 50. * p as f32))
                .scale(glam::Vec2::new(scl, scl))
        );

    } 
    

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

    if animation_state.selected_item_high_score.1 == 1 {
        let arrow = Text::new(TextFragment{
            text: ">".to_string(),
            font: Some("Tetris font".to_string()),
            color: Some(Color::WHITE), 
            scale: Some(PxScale::from(60.0))
    
        });
    
        canvas.draw(&arrow,
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(center.0 - image_half_size.0 + 100., center.1 + 225.))
                .scale(glam::Vec2::new(scl, scl))
        );
    }
}