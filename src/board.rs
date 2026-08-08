use raylib::prelude::*;

pub fn draw(d: &mut RaylibDrawHandle) {
    let tile_size = 100;

    for row in 0..8 {
        for column in 0..8 {
            let screen_x = column * tile_size;
            let screen_y = row * tile_size;

            let color = if (row + column) % 2 == 0 {
                Color::BEIGE
            } else {
                Color::BLACK
            };

            d.draw_rectangle(screen_x, screen_y, 100, 100, color);
        }
    }

    // Files (a-h)
    let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    for (column, letter) in files.iter().enumerate() {
        let x = column as i32 * tile_size + 5;
        let y = 800 - 20;

        d.draw_text(&letter.to_string(), x, y, 20, Color::DARKGRAY);
    }

    // Ranks (8-1)
    for row in 0..8 {
        let x = 5;
        let y = row * tile_size + 5;

        d.draw_text(&(8 - row).to_string(), x, y, 20, Color::DARKGRAY);
    }
}
