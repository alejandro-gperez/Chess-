use raylib::prelude::*;

pub fn draw(d: &mut RaylibDrawHandle) {

    let tile_size = 100;

    for row in 0..8{
            for column in 0..8{

                let screen_x = column * tile_size;
                let screen_y = row * tile_size;

                let color =
                if (row + column) % 2 == 0 {
                    Color::BEIGE
                } else {
                    Color::BLACK
                };

                d.draw_rectangle(
                    screen_x,
                    screen_y,
                    100,
                    100,
                    color,
                );
            }
        }
}