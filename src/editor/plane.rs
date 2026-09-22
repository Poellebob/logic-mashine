use raylib::prelude::*;

use crate::editor;

pub fn draw(d: &mut RaylibDrawHandle, app: &editor::App) {
    let mut plane = d.begin_mode2D(app.camera);
}
