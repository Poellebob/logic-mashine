use raylib::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct Layout {
    pub position: Vector2,
    pub size: Vector2,
}

pub trait Item {
    fn position(self, x: f32, y: f32) -> Self;
    fn size(self, x: f32, y: f32) -> Self;
}

pub trait HasLayout {
    fn layout_mut(&mut self) -> &mut Layout;
}

impl<T: HasLayout> Item for T {
    fn position(mut self, x: f32, y: f32) -> Self {
        let l = self.layout_mut();
        l.position.x = x;
        l.position.y = y;
        self
    }

    fn size(mut self, x: f32, y: f32) -> Self {
        let l = self.layout_mut();
        l.size.x = x;
        l.size.y = y;
        self
    }
}

pub struct Container {
    layout: Layout,
}

impl HasLayout for Container {
    fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
}

pub struct Rect {
    layout: Layout,
    color: Color,
    roundness: f32,
}

impl HasLayout for Rect {
    fn layout_mut(&mut self) -> &mut Layout {
        &mut self.layout
    }
}

impl Rect {
    pub fn new() -> Self {
        Rect {
            layout: Layout::default(),
            color: Color::new(0, 0, 0, 255),
            roundness: 0.0,
        }
    }

    pub fn color(mut self, c: Color) -> Self {
        self.color = c;
        self
    }

    pub fn roundness(mut self, r: f32) -> Self {
        self.roundness = r;
        self
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_rounded(
            Rectangle::new(
                self.layout.position.x,
                self.layout.position.y,
                self.layout.size.x,
                self.layout.size.y,
            ),
            self.roundness,
            1,
            self.color,
        );
    }
}
