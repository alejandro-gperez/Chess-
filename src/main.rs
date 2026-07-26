use raylib::prelude::*;

mod board;
mod pieces;
mod position;

fn main(){
    let (mut rl, thread) = raylib::init() //Window config, rl is raylib's handler and the therad is a 'key' to let us draw.
    .size(800, 800)
    .title("Chess")
    .build(); //Builds window

    let position = position::Position::new();

    while !rl.window_should_close(){ //Loop that draws the window, 60 times per second. Clears, draws, renders.
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        board::draw(&mut d);

        position.draw(&mut d);

    }
}