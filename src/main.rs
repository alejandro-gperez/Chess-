use raylib::prelude::*;
use crate::pieces::Piece;

mod board;
mod pieces;
mod position;
mod textures;
mod movement;

struct DraggingPiece {
    row: usize,
    col: usize,
    piece: Piece,
    offset_x: f32,
    offset_y: f32,
}

fn main(){
    let (mut rl, thread) = raylib::init() //Window config, rl is raylib's handler and the therad is a 'key' to let us draw.
    .size(800, 800)
    .title("Chess")
    .build(); //Builds window

    let mut position = position::Position::new();

    let textures = textures::PieceTextures::new(&mut rl, &thread);

    let mut dragging: Option<DraggingPiece> = None;

while !rl.window_should_close() {

    let mouse = rl.get_mouse_position();

    let row = (mouse.y / 100.0) as usize;
    let column = (mouse.x / 100.0) as usize;

    // Drag begins
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {

        if row < 8 && column < 8 && position.board[row][column].is_some() {
            let piece = position.board[row][column].unwrap();

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

        println!("Agarré una pieza");
    }
}

   // Drag stops
if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {

    if let Some(drag) = dragging.take() {

        let legal_moves = movement::legal_moves(
            &drag.piece,
            &position,
            drag.row,
            drag.col,
        );

        if row < 8 && column < 8 && legal_moves.contains(&(row, column)) {

            position.move_piece(
                drag.row,
                drag.col,
                row,
                column,
            );
        }
    }
}
    //resetting
    if rl.is_key_pressed(KeyboardKey::KEY_R) {
    position = position::Position::new();
    dragging = None;
    }

    let dragging_square = dragging
        .as_ref()
        .map(|drag| (drag.row, drag.col));

    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);

    board::draw(&mut d);
    position.draw(&mut d, dragging_square, &textures);
    if let Some(drag) = &dragging {

        let draw_x = mouse.x - drag.offset_x;
        let draw_y = mouse.y - drag.offset_y;

        pieces::draw_piece_pixels(
            &drag.piece,
            draw_x as i32,
            draw_y as i32,
            &mut d,
            &textures,
        );
    }

 }
}