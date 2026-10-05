use raylib::ffi::{CSSPalette, RaylibPalette};
use raylib::prelude::*;

use crate::editor;
use crate::editor::ui::{Collumn, HasItem, HasRectangle, Item};

pub fn draw(d: &mut RaylibDrawHandle, app: &editor::App) {
    let screen_w = d.get_screen_width();
    let screen_h = d.get_screen_height();

    Collumn::new()
        .position(30.0, 30.0)
        .size(100.0, 200.0)
        .spaceing(10.0)
        .child(
            Item::new()
                .radius(20.0)
                .size(300.0, 200.0)
                .color(Color::GREEN),
        )
        .child(
            Item::new()
                .radius(20.0)
                .size(300.0, 200.0)
                .color(Color::BLUE),
        )
        .draw(d);
}
