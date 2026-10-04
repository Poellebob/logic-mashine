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

// properly unnecessary
pub trait HasItem: HasRectangle {
    fn item_mut(&mut self) -> &mut Item;

    fn roundness(mut self, r: f32) -> Self
    where
        Self: Sized,
    {
        let i = self.item_mut();
        i.roundness = r;
        self
    }

    fn color(mut self, color: Color) -> Self
    where
        Self: Sized,
    {
        let i = self.item_mut();
        i.color = color;
        self
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle)
    where
        Self: Sized,
    {
        d.draw_rectangle_rounded(
            *self.rectangle_mut(),
            self.item_mut().roundness,
            2,
            self.item_mut().color,
        )
    }
}

pub struct Item {
    rectangle: Rectangle,
    roundness: f32,
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
            roundness: 0.0,
            color: Color::default(),
        }
    }

    pub fn roundness(mut self, r: f32) -> Self {
        self.roundness = r;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

pub struct Collumn {
    children: Vec<Box<dyn HasItem>>,
    rectangle: Rectangle,
    spaceing: f32,
}

impl Collumn {
    fn new() -> Self {
        Collumn {
            children: vec![],
            rectangle: Rectangle::default(),
            spaceing: 0.0,
        }
    }

    fn child(mut self, child: impl HasItem + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle) {
        if self.children.len() == 0 {
            return;
        };

        let mut positioner: f32 = 0.0;

        for child in &mut self.children {
            let mut child_rec = *child.rectangle_mut();

            child_rec.x = self.rectangle.x;
            child_rec.y = self.rectangle.y + positioner;

            positioner += child_rec.height + self.spaceing;

            d.draw_rectangle_lines_ex(child_rec, 2.0, child.item_mut().color);
        }
    }
}
