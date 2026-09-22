use raylib::prelude::*;

use crate::editor;

pub fn draw(d: &mut RaylibDrawHandle, app: &editor::App) {
    let screen_w = d.get_screen_width();
    let screen_h = d.get_screen_height();

    // FPS counter
    d.draw_text(
        &format!(
            "cam: x: {}, y: {}",
            app.camera.target.x, app.camera.target.y
        ),
        10,
        10,
        20,
        Color::BLACK,
    );

    // Bottom status bar
    let bar_h = 30;
    d.draw_rectangle(
        0,
        screen_h - bar_h,
        screen_w,
        bar_h,
        Color::new(0, 0, 0, 180),
    );
    d.draw_text("Logic Machine", 10, screen_h - bar_h + 7, 16, Color::WHITE);
}
