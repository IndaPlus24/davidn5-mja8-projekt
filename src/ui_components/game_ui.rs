use std::collections::HashMap;

use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextFragment, TextLayout}};

use crate::{consts::BoardRenderType, Game, PieceType};
use crate::consts::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS};


impl Game {
    pub fn render_board(&mut self, assets: &HashMap<String, Image>, canvas: &mut Canvas, pos: (f32, f32), scl: f32) {
        canvas.draw(
            assets.get("hold").unwrap(),
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(pos.0, pos.1))
                .scale(glam::Vec2::new(scl, scl))
        );

        let mut x_offset = 164. * scl;
        if self.battle_mode {
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
        let x = pos.0 + if self.battle_mode {204. * scl} else {168. * scl};
        let y = pos.1 + 608. * scl;

        for r in 0..BOARD_AMOUNT_ROWS {
            for c in 0..BOARD_AMOUNT_COLUMNS {
                if let Some(piece_type) = self.board[r][c] {
                    let image = assets.get(&piece_type).unwrap();
                    canvas.draw(
                        image,
                        graphics::DrawParam::new()
                            .dest(glam::Vec2::new(
                                x + c as f32 * 32. * scl,
                                y - r as f32 * 32. * scl
                            ))
                            .scale(glam::Vec2::new(scl, scl))
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
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(
                        x + (mc + dc) as f32 * 32. * scl,
                        y - (mr + dr) as f32 * 32. * scl
                    ))
                    .scale(glam::Vec2::new(scl, scl))
            );
        });

        //Ghost piece
        let ghost_piece = self.get_ghost_piece();
        let image = assets.get(&self.active_piece.piece_type).unwrap();

        let (mr, mc) = ghost_piece.midpoint;
        ghost_piece.block_positions.iter().for_each(|(dr, dc)| {
            // SET POSITION AND OPACITY
            let param = graphics::DrawParam::new()
                .dest(glam::Vec2::new(
                    x + (mc + dc) as f32 * 32. * scl,
                    y - (mr + dr) as f32 * 32. * scl
                ))
                .scale(glam::Vec2::new(scl, scl))
                .color(graphics::Color::from_rgba(255, 255, 255, 15));

            canvas.draw(image, param);
        });

        //Hold piece
        if let Some(hold_piece) = &self.held_piece {
            let piece_type = if self.can_hold {hold_piece.piece_type} else {PieceType::X};
            let image = assets.get(&piece_type).unwrap();

            let (mut x, mut y) = (pos.0 + 68. * scl, pos.1 + 80. * scl);
            let (x_offset, y_offset) = get_piece_offset(hold_piece.piece_type, scl);
            x += x_offset;
            y += y_offset;

            hold_piece.block_positions.iter().for_each(|(dr, dc)| {
                canvas.draw(
                    image,
                    graphics::DrawParam::new()
                        .dest(glam::Vec2::new(
                            x + *dc as f32 * 32. * scl,
                            y - *dr as f32 * 32. * scl
                        ))
                        .scale(glam::Vec2::new(scl, scl))
                );
            });
        }

        //Next queue
        let (mut x, mut y) = (pos.0 + 556. * scl, pos.1 + 80. * scl);
        if self.battle_mode {x += 36. * scl};
        
        for i in 0..5 {
            let next_piece = &self.piece_queue[i];
            let (x_offset, y_offset) = get_piece_offset(next_piece.piece_type, scl);
            x += x_offset;
            y += y_offset;

            let image = assets.get(&next_piece.piece_type).unwrap();
            next_piece.block_positions.iter().for_each(|(dr, dc)| {
                canvas.draw(
                    image,
                    graphics::DrawParam::new()
                        .dest(glam::Vec2::new(
                            x + *dc as f32 * 32. * scl,
                            y - *dr as f32 * 32. * scl
                        ))
                        .scale(glam::Vec2::new(scl, scl))
                );
            });

            x -= x_offset;
            y -= y_offset;
            y += 96.;
        }
    }

    // Render different stats depending on gamemode
    pub fn render_stats(&mut self, canvas: &mut Canvas, pos: (f32, f32), scl: f32) {
        match self.render_type {
            BoardRenderType::Marathon => {
                let mut score = Text::new(TextFragment{
                    text: self.score.to_string(),
                    font: Some("Tetris font".to_string()),
                    color: Some(Color::WHITE), 
                    scale: Some(PxScale::from(24.))
                });
                score.set_layout(TextLayout::center());

                canvas.draw(&score,
                    graphics::DrawParam::new()
                        .dest(glam::Vec2::new(pos.0 + 328., pos.1 + 668.))
                        .scale(glam::Vec2::new(scl, scl))
                );
            },
            BoardRenderType::FourtyLines => {

            },
            BoardRenderType::Versus => {

            },
        }
    }
}

fn get_piece_offset(piece_type: PieceType, scl: f32) -> (f32, f32) {
    match piece_type {
        PieceType::I => (-16. * scl, -16. * scl),
        PieceType::O => (-16. * scl, 0.),
        _ => (0., 0.),
    }
}