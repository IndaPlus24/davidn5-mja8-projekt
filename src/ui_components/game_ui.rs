use std::{collections::HashMap, time::Duration};

use ggez::{glam, graphics::{self, Canvas, Color, Image, PxScale, Text, TextAlign, TextFragment, TextLayout}};

use crate::{consts::{GameMode, GARBAGE_CAP}, Game, Piece, PieceType};
use crate::consts::{BOARD_AMOUNT_COLUMNS, BOARD_AMOUNT_ROWS};


impl Game {
    pub fn render_board(&mut self, assets: &HashMap<String, Image>, canvas: &mut Canvas) -> &mut Self {
        let pos = self.canvas_pos;
        let scl = self.canvas_scl;

        let (mut x, y) = (pos.0, pos.1);
        canvas.draw(
            assets.get("hold").unwrap(),
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(x, y))
                .scale(glam::Vec2::new(scl, scl))
        );

        x += 164. * scl;
        if self.gamemode == GameMode::Versus {
            canvas.draw(
                assets.get("garb_bar").unwrap(),
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(x, y))
                    .scale(glam::Vec2::new(scl, scl))
            );

            // Garbage queue
            let mut y_offset = 608. * scl;
            let mut draw_garbage = |img: &Image, col: Color, pos: (f32, f32)| {
                canvas.draw(
                    img,
                    graphics::DrawParam::new()
                        .dest(glam::Vec2::new(pos.0 + 4. * scl, pos.1))
                        .scale(glam::Vec2::new(scl, scl))
                        .color(col)
                );
            };

            self.garbage_inbound.iter().for_each(|g| {
                let col = match g.2 {
                    Some(_) => Color::from_rgba(255, 255, 255, 127),
                    None => Color::WHITE
                };

                if g.1 == 1 {
                    draw_garbage(assets.get("garb_s").unwrap(), col, (x, y + y_offset));
                    y_offset -= 32. * scl;
                } else {
                    for i in 0..g.1 {
                        if i == 0 {
                            draw_garbage(assets.get("garb_b").unwrap(), col, (x, y + y_offset));
                        }
                        else if i == g.1 - 1 {
                            draw_garbage(assets.get("garb_t").unwrap(), col, (x, y + y_offset));
                        } else {
                            draw_garbage(assets.get("garb_m").unwrap(), col, (x, y + y_offset));
                        }
                        y_offset -= 32. * scl;
                    }
                }
            });

            let y_offset = (639. - 32. * GARBAGE_CAP as f32) * scl;
            canvas.draw(
                assets.get("garb_sep").unwrap(),
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(x + 8. * scl, y + y_offset))
                    .scale(glam::Vec2::new(scl, scl))
            );
            
            x += 36. * scl;
        }

        canvas.draw(
            assets.get("main").unwrap(),
            graphics::DrawParam::new()
                .dest(glam::Vec2::new(x, y))
                .scale(glam::Vec2::new(scl, scl))
        );

        self
    }

    pub fn render_pieces(&mut self, assets: &HashMap<PieceType, Image>, canvas: &mut Canvas) -> &mut Self {
        let pos = self.canvas_pos;
        let scl = self.canvas_scl;

        //Board cells
        let (mut x, y) = (pos.0 + 168. * scl, pos.1 + 608. * scl);
        if self.gamemode == GameMode::Versus {x += 36. * scl};

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
        if let Some(piece_type) = self.held_piece {
            let piece_texture = if self.can_hold {piece_type} else {PieceType::X};
            let image = assets.get(&piece_texture).unwrap();

            let (mut x, mut y) = (pos.0 + 68. * scl, pos.1 + 80. * scl);
            let (x_offset, y_offset) = get_piece_offset(piece_type, scl);
            x += x_offset;
            y += y_offset;

            Piece::get_block_positions(piece_type, 0).iter().for_each(|(dr, dc)| {
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
        if self.gamemode == GameMode::Versus {x += 36. * scl};
        
        for i in 0..5 {
            let piece_type = self.piece_queue[i];
            let (x_offset, y_offset) = get_piece_offset(piece_type, scl);
            x += x_offset;
            y += y_offset;

            let image = assets.get(&piece_type).unwrap();
            Piece::get_block_positions(piece_type, 0).iter().for_each(|(dr, dc)| {
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
            y += 96. * scl;
        }

        self
    }

    pub fn render_misc(&mut self, assets: &HashMap<String, Image>, canvas: &mut Canvas) -> &mut Self {
        
        let pos = self.canvas_pos;
        let scl = self.canvas_scl;
        let (mut x, y) = (pos.0 + 168. * scl, pos.1);
        if self.gamemode == GameMode::Versus {x += 36. * scl}

        
        // Countdown
        if let Some(start) = self.countdown_start {
            let elapsed = start.elapsed();
            let remaining = self.countdown_duration.checked_sub(elapsed).unwrap_or(Duration::ZERO);
            let seconds_left = remaining.as_secs();
    
            let countdown_text_str = match seconds_left {
                3 => "3",
                2 => "2",
                1 => "1",
                0 => "Go!",
                _ => "",
            };

            let mut countdown_text = Text::new(TextFragment {
                text: countdown_text_str.to_string(),
                font: Some("Tetris font".to_string()),
                color: Some(Color::WHITE),
                scale: Some(PxScale::from(40.)),
            });
            countdown_text.set_layout(TextLayout {
                h_align: TextAlign::Middle,
                v_align: TextAlign::Begin,
            });
            let (x, y) = (pos.0 + 168. * scl + 6. * 32. , pos.1 + 288. * scl);


            canvas.draw(
                &countdown_text,
                graphics::DrawParam::new()
                    .dest(glam::Vec2::new(x,y))
                    .scale(glam::Vec2::new(scl, scl)),
            );

        }

        // Line marker
        match self.gamemode {
            GameMode::FourtyLines |
            GameMode::Marathon => {
                let mut lines_left = -(self.lines as isize);
                if self.gamemode == GameMode::Marathon {lines_left += 150}
                else {lines_left += 40}
                
                if lines_left <= 20 && lines_left > 0 {
                    let y_offset = 632. - (lines_left as f32 * 32.);
                    canvas.draw(
                        assets.get("line_marker").unwrap(),
                        graphics::DrawParam::new()
                            .dest(glam::Vec2::new(x, y + y_offset * scl))
                            .scale(glam::Vec2::new(scl, scl))
                    );
                }
            }
            _ => ()
        }

        if self.game_over {
            if self.objective_completed {
                canvas.draw(
                    assets.get("finish").unwrap(),
                    graphics::DrawParam::new()
                        .dest(glam::Vec2::new(x, y + 160. * scl))
                        .scale(glam::Vec2::new(scl, scl))
                );
            } else {
                canvas.draw(
                    assets.get("game_over").unwrap(),
                    graphics::DrawParam::new()
                        .dest(glam::Vec2::new(x, y + 160. * scl))
                        .scale(glam::Vec2::new(scl, scl))
                );
            }    
        }

        self
    }

    // Render different stats depending on gamemode
    pub fn render_stats(&mut self, canvas: &mut Canvas) -> &mut Self {
        let pos = self.canvas_pos;
        let scl = self.canvas_scl;

        match self.gamemode {
            GameMode::Marathon => {
                self.render_marathon_stats(canvas, pos, scl);
            }
            GameMode::FourtyLines => {
                self.render_40l_stats(canvas, pos, scl);
            }
            GameMode::Survival => {
                self.render_survival_stats(canvas, pos, scl);
            }
            GameMode::Versus => {
                self.render_vs_stats(canvas, pos, scl);
            }
        }

        self
    }
}

fn get_piece_offset(piece_type: PieceType, scl: f32) -> (f32, f32) {
    match piece_type {
        PieceType::I => (-16. * scl, -16. * scl),
        PieceType::O => (-16. * scl, 0.),
        _ => (0., 0.),
    }
}