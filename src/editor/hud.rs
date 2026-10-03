use raylib::prelude::*;

use crate::editor;
use crate::editor::ui::{Item, Rect};

pub fn draw(d: &mut RaylibDrawHandle, app: &editor::App) {
    let screen_w = d.get_screen_width();
    let screen_h = d.get_screen_height();

    Rect::new()
        .size(200.0, 200.0)
        .position(100.0, 100.0)
        .roundness(0.5)
        .draw(d);
}
