use raylib::{ffi::CSSPalette, prelude::*};

use crate::editor;
use crate::editor::ui;

pub fn draw(d: &mut RaylibDrawHandle, app: &editor::App) {
    let screen_w = d.get_screen_width();
    let screen_h = d.get_screen_height();

    let and_button = Rectangle::new(screen_w as f32 - 100.0, 0.0, 100.0, 100.0);
}
