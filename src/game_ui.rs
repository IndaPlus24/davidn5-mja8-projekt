use std::collections::HashMap;

use ggez::{glam, graphics::{self, Canvas, Image}, Context};

use crate::{Game, PieceType};
use crate::consts::{BOARD_LOWER_LEFT, BLOCK_SIZE, HOLD_PIECE_MIDDLE, BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS, EMPTY_BLOCK_COLOR};


impl Game {
    pub fn render_pieces(&mut self, images: &HashMap<PieceType, Image>, canvas: &mut Canvas, ctx: &mut Context) {
        self.render_board_pieces(images, canvas, ctx);
        self.render_ghost_piece(images, canvas);
        self.render_active_piece(images, canvas);
        self.render_held_piece(images, canvas);
    }

    pub fn render_active_piece(&mut self, images: &HashMap<PieceType, Image>, canvas: &mut Canvas) {
        let image = images.get(&self.active_piece.piece_type).unwrap();
        let (mr, mc) = self.active_piece.midpoint;
        self.active_piece.block_positions.iter().for_each(|(dr, dc)| {
            canvas.draw(
                image,
                graphics::DrawParam::new().dest(glam::Vec2::new(
                    (BOARD_LOWER_LEFT.0 + (mc + dc) as i32 * BLOCK_SIZE + 1) as f32,
                    (BOARD_LOWER_LEFT.1 - (mr + dr) as i32 * BLOCK_SIZE + 1) as f32,
                )),
            );
        });       
    }

    pub fn render_held_piece(&mut self, images: &HashMap<PieceType, Image>, canvas: &mut Canvas) {
        if let Some(held_piece) = &self.held_piece {
            let image = images.get(&held_piece.piece_type).unwrap();

            held_piece.block_positions.iter().for_each(|(dr, dc)| {
                canvas.draw(
                    image,
                    graphics::DrawParam::new().dest(glam::Vec2::new(
                        (HOLD_PIECE_MIDDLE.0 + *dc * (BLOCK_SIZE as isize+ 1)) as f32,
                        (HOLD_PIECE_MIDDLE.1 - *dr * (BLOCK_SIZE as isize + 1)) as f32,
                    )),
                );

            });
        }
    }

    pub fn render_ghost_piece(&mut self, images: &HashMap<PieceType, Image>, canvas: &mut Canvas){
        let ghost_piece = self.get_ghost_piece();
        let image = images.get(&self.active_piece.piece_type).unwrap();

        let (mr, mc) = ghost_piece.midpoint;
        ghost_piece.block_positions.iter().for_each(|(dr, dc)| {

            // SET POSITION AND OPACITY
            let param = graphics::DrawParam::new()
                .dest(glam::Vec2::new(
                    (BOARD_LOWER_LEFT.0 + (mc + dc) as i32 * BLOCK_SIZE + 1) as f32,
                    (BOARD_LOWER_LEFT.1 - (mr + dr) as i32 * BLOCK_SIZE + 1) as f32,
                ))
                .color(graphics::Color::from_rgba(255, 255, 255, 127));

            canvas.draw(image, param);
        });
    }

    pub fn render_board_pieces(&mut self, images: &HashMap<PieceType, Image>, canvas: &mut Canvas, ctx: &mut Context){
        for r in 0..BOARD_AMOUNT_ROWS {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                match self.board[r][c] {
                    Some(piece_type) => {
                        let image = images.get(&piece_type).unwrap();
                        canvas.draw(
                            image,
                            graphics::DrawParam::new().dest(glam::Vec2::new(
                                (BOARD_LOWER_LEFT.0 + c as i32 * BLOCK_SIZE + 1) as f32,
                                (BOARD_LOWER_LEFT.1 - r as i32 * BLOCK_SIZE + 1) as f32,
                            )),
                        );
                    },

                    None => {
                        if r < 20 {
                            let rectangle = graphics::Mesh::new_rectangle(
                                ctx,
                                graphics::DrawMode::fill(),
                                graphics::Rect::new_i32(
                                    BOARD_LOWER_LEFT.0 + c as i32 * BLOCK_SIZE + 1,
                                    BOARD_LOWER_LEFT.1 - r as i32 * BLOCK_SIZE + 1,
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
    }
}