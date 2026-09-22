mod hud;
mod plane;

use raylib::prelude::*;

use crate::core::logic;

#[derive(Default)]
pub struct State {
    gate: logic::Gate,
}

pub struct App {
    pub state: State,
    pub camera: Camera2D,
}

impl App {
    pub fn init() -> Self {
        App {
            state: State::default(),
            camera: Camera2D {
                offset: Vector2::zero(),
                target: Vector2::zero(),
                rotation: 0.0,
                zoom: 1.0,
            },
        }
    }

    pub fn run(&mut self) {
        let (mut rl, thread) =
            raylib::init().title("Hello, World").resizable().build();

        rl.set_target_fps(60);

        while !rl.window_should_close() {
            self.update(&mut rl);

            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);

            plane::draw(&mut d, &self);
            hud::draw(&mut d, &self);
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle) {
        // Camera zoom with scroll wheel
        let scroll = rl.get_mouse_wheel_move();
        if scroll != 0.0 {
            self.camera.zoom *= if scroll > 0.0 { 1.1 } else { 0.9 };
            self.camera.zoom = self.camera.zoom.clamp(0.1, 5.0);
        }

        // Camera pan with middle mouse button
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
            let delta = rl.get_mouse_delta();
            self.camera.target.x -= delta.x / self.camera.zoom;
            self.camera.target.y -= delta.y / self.camera.zoom;
        }
    }
}
