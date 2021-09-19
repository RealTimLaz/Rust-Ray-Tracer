use crate::math::{Color, Point};

use super::{Texture, utils::Perlin};

pub struct PerlinTexture {
    noise: Perlin,
    scale: f64,
}

impl PerlinTexture {

    pub fn new() -> Self {
        let noise: Perlin = Default::default();
        PerlinTexture {
            noise,
            scale: 1.0
        }
    }

    pub fn new_with_scale(scale: f64) -> Self {
        let noise: Perlin = Default::default();
        PerlinTexture {
            noise,
            scale
        }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, _u: f64, _v: f64, p: Point) -> Color {
        Color::ONE * self.noise.noise(self.scale * p)
    }
}

impl Default for PerlinTexture {
    fn default() -> Self {
       PerlinTexture::new()
    }
}