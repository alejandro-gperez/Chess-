use crate::pieces::{Piece, PieceType};
use raylib::prelude::*;

mod board;
mod game;
mod movement;
mod pieces;
mod position;
mod textures;

struct DraggingPiece {
    row: usize,
    col: usize,
    piece: Piece,
    offset_x: f32,
    offset_y: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init() //Window config, rl is raylib's handler and the therad is a 'key' to let us draw.
        .size(800, 800)
        .title("Chess")
        .build(); //Builds window

    let mut game = game::Game::new();

    let textures = textures::PieceTextures::new(&mut rl, &thread);

    let mut dragging: Option<DraggingPiece> = None;

    let mut legal_moves: Vec<(usize, usize)> = Vec::new();

    while !rl.window_should_close() {
        let mouse = rl.get_mouse_position();

        let row = (mouse.y / 100.0) as usize;
        let column = (mouse.x / 100.0) as usize;

        //Promoting mechanic by selecting piece with keys.
        if let Some(_) = game.position.promotion {
            if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
                game.position.promote(PieceType::Queen);
                legal_moves.clear();
                dragging = None;
            }

            if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
                game.position.promote(PieceType::Knight);
                legal_moves.clear();
                dragging = None;
            }

            if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
                game.position.promote(PieceType::Bishop);
                legal_moves.clear();
                dragging = None;
            }

            if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
                game.position.promote(PieceType::Rook);
                legal_moves.clear();
                dragging = None;
            }
        } else {
            //If there's no pawn that can be promoted. if there is, game stops and checks if any of the keys 1-4 has been pressed.

            // Drag begins
            if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                if row < 8 && column < 8 {
                    if let Some(piece) = game.position.board[row][column] {
                        if game.can_select(piece.piece_color) {
                            let piece_x = column as f32 * 100.0;
                            let piece_y = row as f32 * 100.0;

                            let offset_x = mouse.x - piece_x;
                            let offset_y = mouse.y - piece_y;

                            dragging = Some(DraggingPiece {
                                row,
                                col: column,
                                piece,
                                offset_x,
                                offset_y,
                            });

                            legal_moves =
                                movement::legal_moves(&piece, &game.position, row, column);

                            println!("Agarré una pieza");
                        }
                    }
                }
            }

            // Drag stops
            if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                if let Some(drag) = dragging.take() {
                    if row < 8 && column < 8 && legal_moves.contains(&(row, column)) {
                        game.move_piece(drag.row, drag.col, row, column);

                        if game.should_switch_turns() {
                            game.next_turn();
                        }
                    }
                }

                legal_moves.clear();
            }
        }

        // resetting
        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            game = game::Game::new();
            dragging = None;
        }

        let dragging_square = dragging.as_ref().map(|drag| (drag.row, drag.col));

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        board::draw(&mut d);

        for &(row, col) in &legal_moves {
            let x = col as i32 * 100 + 50;
            let y = row as i32 * 100 + 50;

            d.draw_text("*", x - 8, y - 20, 40, Color::GREEN);
        }

        game.position.draw(&mut d, dragging_square, &textures);

        if game.position.promotion.is_some() {
            d.draw_text(
                "1: Queen  2: Knight  3: Bishop  4: Rook",
                80,
                10,
                24,
                Color::RED,
            );
        }

        if let Some(drag) = &dragging {
            let draw_x = mouse.x - drag.offset_x;
            let draw_y = mouse.y - drag.offset_y;

            pieces::draw_piece_pixels(&drag.piece, draw_x as i32, draw_y as i32, &mut d, &textures);
        }
    }
}
