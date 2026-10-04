use raylib::ffi::CSSPalette;
use raylib::prelude::*;

use crate::editor;
use crate::editor::ui::{Collumn, HasItem, HasRectangle, Item};

pub fn draw(d: &mut RaylibDrawHandle, app: &editor::App) {
    let screen_w = d.get_screen_width();
    let screen_h = d.get_screen_height();

    Item::new()
        .position(30.0, 30.0)
        .size(100.0, 200.0)
        .color(Color::BLACK)
        .roundness(1.0)
        .draw(d);
}
