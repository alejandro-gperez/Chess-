use raylib::prelude::*;
use crate::pieces::Piece;

mod board;
mod pieces;
mod position;

fn main(){
    let (mut rl, thread) = raylib::init() //Window config, rl is raylib's handler and the therad is a 'key' to let us draw.
    .size(800, 800)
    .title("Chess")
    .build(); //Builds window

    let mut position = position::Position::new();

let mut dragging: Option<(usize, usize, Piece)> = None;

while !rl.window_should_close() {

    let mouse = rl.get_mouse_position();

    let row = (mouse.y / 100.0) as usize;
    let column = (mouse.x / 100.0) as usize;

    // Drag begins
    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {

        if position.board[row][column].is_some() {
            let piece = position.board[row][column].unwrap();
            dragging = Some((row, column, piece));
            println!("Agarré una pieza");
        }
    }

    // Drag stops
    if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {

        if let Some((from_row, from_col, _)) = dragging {
            position.move_piece(from_row, from_col, row, column);
            dragging = None;
            println!("Solté la pieza");
        }
    }

    //resetting
    if rl.is_key_pressed(KeyboardKey::KEY_R) {
    position = position::Position::new();
    dragging = None;
    }

    let dragging_square = dragging.map(|(row, column, _)| (row, column));

    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);

    board::draw(&mut d);
    position.draw(&mut d, dragging_square);
    if let Some((_, _, piece)) = dragging {
        pieces::draw_piece_pixels(
            &piece,
            mouse.x as i32,
            mouse.y as i32,
            &mut d,
        );
        }

    }
}