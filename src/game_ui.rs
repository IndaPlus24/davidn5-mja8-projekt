use std::collections::HashMap;

use ggez::{glam, graphics::{self, Canvas, Image}, Context};

use crate::Game;
use crate::consts::{BOARD_UPPER_LEFT, BLOCK_SIZE, HOLD_PIECE_MIDDLE, BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS, EMPTY_BLOCK_COLOR};


impl Game {
    pub fn render_active_piece(&mut self, image : &Image, canvas : &mut Canvas){
        let (mr, mc) = self.active_piece.midpoint;
        self.active_piece.block_positions.iter().for_each(|(dr, dc)| {
            canvas.draw(
                image,
                graphics::DrawParam::new().dest(glam::Vec2::new(
                    (BOARD_UPPER_LEFT.0 + (mc + dc) as i32 * BLOCK_SIZE + 1) as f32,
                    (BOARD_UPPER_LEFT.1 + (mr + dr) as i32 * BLOCK_SIZE + 1) as f32,
                )),
            );
        });       
    }

    pub fn render_held_piece(&mut self,images : &HashMap<String, Image>, canvas : &mut Canvas) {
        if let Some(held_piece) = &self.held_piece {

            let path = &self.active_piece.piece_type.get_path();
        let image = images.get(path).unwrap();

            held_piece.block_positions.iter().for_each(|(dr, dc)| {
                canvas.draw(
                    image,
                    graphics::DrawParam::new().dest(glam::Vec2::new(
                        (HOLD_PIECE_MIDDLE.0 + *dc * (BLOCK_SIZE as isize+ 1)) as f32,
                        (HOLD_PIECE_MIDDLE.1 + *dr * (BLOCK_SIZE as isize + 1)) as f32,
                    )),
                );

            });
        }
    }

    pub fn render_ghost_piece(&mut self, image : &Image,canvas : &mut Canvas){
        let ghost_piece = self.board.get_ghost_piece(&self.active_piece);

        let (mr, mc) = ghost_piece.midpoint;
        ghost_piece.block_positions.iter().for_each(|(dr, dc)| {
            canvas.draw(
                image,
                graphics::DrawParam::new().dest(glam::Vec2::new(
                    (BOARD_UPPER_LEFT.0 + (mc + dc) as i32 * BLOCK_SIZE + 1) as f32,
                    (BOARD_UPPER_LEFT.1 + (mr + dr) as i32 * BLOCK_SIZE + 1) as f32,
                )),
            );
        });
    }

    pub fn render_board(&mut self, images : &HashMap<String, Image>,canvas : &mut Canvas, ctx: &mut Context){
        for r in 0..BOARD_AMOUNT_ROWS {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                if self.board.table[r][c].is_occupied() {
                    let path = &self.board.table[r][c].path;
                    let image = images.get(path).unwrap();
                    canvas.draw(
                        image,
                        graphics::DrawParam::new().dest(glam::Vec2::new(
                            (BOARD_UPPER_LEFT.0 + c as i32 * BLOCK_SIZE + 1) as f32,
                            (BOARD_UPPER_LEFT.1 + r as i32 * BLOCK_SIZE + 1) as f32,
                        )),
                    );
                } else {
                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new_i32(
                            BOARD_UPPER_LEFT.0 + c as i32 * BLOCK_SIZE + 1,
                            BOARD_UPPER_LEFT.1 + r as i32 * BLOCK_SIZE + 1,
                            BLOCK_SIZE - 2,
                            BLOCK_SIZE - 2,
                        ),
                        EMPTY_BLOCK_COLOR,
                    )
                    .expect("COULDNT CREATE RECTANGLE FROM BLOCK");

                    canvas.draw(&rectangle, graphics::DrawParam::default());
                }
            }
        }
    }
}