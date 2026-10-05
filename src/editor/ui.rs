use raylib::{
    ffi::{CSSPalette, Rectangle},
    prelude::*,
};

pub trait HasRectangle {
    fn rectangle_mut(&mut self) -> &mut Rectangle;

    fn position(mut self, x: f32, y: f32) -> Self
    where
        Self: Sized,
    {
        let r = self.rectangle_mut();
        r.x = x;
        r.y = y;
        self
    }

    fn size(mut self, width: f32, height: f32) -> Self
    where
        Self: Sized,
    {
        let r = self.rectangle_mut();
        r.height = height;
        r.width = width;
        self
    }
}

enum Corners {
    Roundness(f32),
    Radius(f32),
}

// properly unnecessary
pub trait HasItem: HasRectangle {
    fn item_mut(&mut self) -> &mut Item;

    fn roundness(mut self, r: f32) -> Self
    where
        Self: Sized,
    {
        self.item_mut().corners = Corners::Roundness(r);
        self
    }

    fn radius(mut self, r: f32) -> Self
    where
        Self: Sized,
    {
        self.item_mut().corners = Corners::Radius(r);
        self
    }

    fn color(mut self, color: Color) -> Self
    where
        Self: Sized,
    {
        self.item_mut().color = color;
        self
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let rect = *self.rectangle_mut();
        let item = self.item_mut();
        let roundness = match item.corners {
            Corners::Roundness(r) => r,
            Corners::Radius(r) => r / (rect.width.min(rect.height) / 2.0),
        };
        d.draw_rectangle_rounded(rect, roundness, 2, item.color);
    }
}

pub struct Item {
    rectangle: Rectangle,
    corners: Corners,
    color: Color,
}

impl HasItem for Item {
    fn item_mut(&mut self) -> &mut Item {
        self
    }
}

impl HasRectangle for Item {
    fn rectangle_mut(&mut self) -> &mut Rectangle {
        &mut self.rectangle
    }
}

impl Item {
    pub fn new() -> Self {
        Item {
            rectangle: Rectangle::default(),
            corners: Corners::Roundness(0.0),
            color: Color::default(),
        }
    }
}

pub struct Collumn {
    children: Vec<Box<dyn HasItem>>,
    rectangle: Rectangle,
    spaceing: f32,
}

impl HasRectangle for Collumn {
    fn rectangle_mut(&mut self) -> &mut Rectangle {
        &mut self.rectangle
    }
}

impl Collumn {
    pub fn new() -> Self {
        Collumn {
            children: vec![],
            rectangle: Rectangle::default(),
            spaceing: 0.0,
        }
    }

    pub fn spaceing(mut self, space: f32) -> Self {
        self.spaceing = space;
        self
    }

    pub fn child(mut self, child: impl HasItem + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        if self.children.len() == 0 {
            return;
        };

        let mut positioner: f32 = 0.0;

        for child in &mut self.children {
            let r = child.rectangle_mut();

            r.x = self.rectangle.x;
            r.y = self.rectangle.y + positioner;

            positioner += r.height + self.spaceing;

            child.draw(d)
        }
    }
}
