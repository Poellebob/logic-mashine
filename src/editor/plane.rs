use raylib::prelude::*;

use crate::editor;

pub fn draw(d: &mut RaylibDrawHandle, app: &editor::App) {
    let mut d2 = d.begin_mode2D(app.camera);

    // Draw a reference grid
    for x in (-2000..2000).step_by(50) {
        d2.draw_line(x, -2000, x, 2000, Color::LIGHTGRAY);
    }
    for y in (-2000..2000).step_by(50) {
        d2.draw_line(-2000, y, 2000, y, Color::LIGHTGRAY);
    }

    // Origin marker
    d2.draw_line(-20, 0, 20, 0, Color::RED);
    d2.draw_line(0, -20, 0, 20, Color::GREEN);
}
