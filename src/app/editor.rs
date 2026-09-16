use raylib::{ffi::*, prelude::*};

pub fn run() {
    let (mut rl, thread) =
        raylib::init().title("Hello, World").resizable().build();

    rl.set_target_fps(60);

    let mut text_loc = Vector2::new(40.0, 40.0);

    while !rl.window_should_close() {
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            text_loc = rl.get_mouse_position();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text(
            "Hello, World!",
            text_loc.x.round() as i32,
            text_loc.y.round() as i32,
            40,
            Color::BLACK,
        );
    }
}
