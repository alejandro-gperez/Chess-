use raylib::prelude::*;

fn main(){
    let (mut rl, thread) = raylib::init() //Window config, rl is raylib's handler and the therad is a 'key' to let us draw.
    .size(800, 800)
    .title("Chess")
    .build(); //Builds window

    while !rl.window_should_close(){ //Loop that draws the window, 60 times per second. Clears, draws, renders.
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        for row in 0..8{
            for column in 0..8{

                let x = row * 100;
                let y = column * 100;

                let color =
                if (row + column) % 2 == 0 {
                    Color::BEIGE
                } else {
                    Color::BLACK
                };

                d.draw_rectangle(
                    x,
                    y,
                    100,
                    100,
                    color,
                );
            }
        }
    }
}