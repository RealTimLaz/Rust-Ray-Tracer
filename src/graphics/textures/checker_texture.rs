use crate::math::{Color, Point};

use super::{SolidColor, Texture};

pub struct CheckerTexture {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        CheckerTexture {
            even,
            odd,
        }
    }

    pub fn new_from_colors(c1: Color, c2: Color) -> Self {
        CheckerTexture::new(
            Box::new(SolidColor::new(c1)),
            Box::new(SolidColor::new(c2))
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}