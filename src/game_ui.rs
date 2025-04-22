use std::collections::HashMap;

use ggez::{glam, graphics::{self, Canvas, Image}, Context};

use crate::{Game, PieceType};
use crate::consts::{BOARD_LOWER_LEFT, BLOCK_SIZE, HOLD_PIECE_MIDDLE, BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS, EMPTY_BLOCK_COLOR};


impl Game {
    pub fn render_board(&mut self, assets: &HashMap<String, Image>, canvas: &mut Canvas, pos: (f32, f32), scl: f32) {
        canvas.draw(
            assets.get("hold").unwrap(),
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0, pos.1))
                .scale(glam::Vec2::new(scl, scl))
        );

        let mut x_offset = 164. * scl;
        if self.can_recieve_garbage {
            canvas.draw(
                assets.get("garb_bar").unwrap(),
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(pos.0 + x_offset, pos.1))
                    .scale(glam::Vec2::new(scl, scl))
            );
            // TODO: add garbage meter
            canvas.draw(
                assets.get("garb_sep").unwrap(),
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(pos.0 + x_offset + 6. * scl, pos.1 + 383. * scl))
                    .scale(glam::Vec2::new(scl, scl))
            );
            
            x_offset += 36. * scl;
        }

        canvas.draw(
            assets.get("main").unwrap(),
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0 + x_offset, pos.1))
                .scale(glam::Vec2::new(scl, scl))
        );
    }

    pub fn render_pieces(&mut self, assets: &HashMap<PieceType, Image>, canvas: &mut Canvas, pos: (f32, f32), scl: f32) {
        //Board cells
        let x = pos.0 + if self.can_recieve_garbage {208. * scl} else {168. * scl};
        let y = pos.1 + 608. * scl;

        for r in 0..BOARD_AMOUNT_ROWS {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                if let Some(piece_type) = self.board[r][c] {
                    let image = assets.get(&piece_type).unwrap();
                    canvas.draw(
                        image,
                        graphics::DrawParam::new().dest(glam::Vec2::new(
                            x + c as f32 * 32. * scl,
                            y - r as f32 * 32. * scl
                        )),
                    );
                }
            }
        }

        //Active piece
        let image = assets.get(&self.active_piece.piece_type).unwrap();
        let (mr, mc) = self.active_piece.midpoint;
        self.active_piece.block_positions.iter().for_each(|(dr, dc)| {
            canvas.draw(
                image,
                graphics::DrawParam::new().dest(glam::Vec2::new(
                    x + (mc + dc) as f32 * 32. * scl,
                    y - (mr + dr) as f32 * 32. * scl
                )),
            );
        });

        
        //self.render_board_pieces(assets, canvas, ctx);
        //self.render_ghost_piece(assets, canvas);
        //self.render_active_piece(assets, canvas);
        //self.render_held_piece(assets, canvas);
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