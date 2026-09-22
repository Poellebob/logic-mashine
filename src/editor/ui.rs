use raylib::{ffi::CSSPalette, prelude::*};

pub trait Item {
    fn position(&self) -> Vector2;
    fn size(&self) -> Vector2;
    fn draw(&self, d: &mut RaylibDrawHandle);
}

pub struct Rect {
    position: Vector2,
    size: Vector2,
    color: Color,
    radius: f32,
}

impl Item for Rect {
    fn position(&self) -> Vector2 {
        self.position
    }

    fn size(&self) -> Vector2 {
        self.size
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rounded(
            Rectangle::new(
                self.position.x,
                self.position.y,
                self.size.x,
                self.size.y,
            ),
            self.radius,
            1,
            self.color,
        );
    }
}
