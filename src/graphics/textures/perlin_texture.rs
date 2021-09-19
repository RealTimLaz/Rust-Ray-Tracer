use crate::math::{Color, Point};

use super::{Texture, utils::Perlin};

pub struct PerlinTexture {
    noise: Perlin,
}

impl PerlinTexture {

    pub fn new() -> Self {
        let noise: Perlin = Default::default();
        PerlinTexture {
            noise
        }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, _u: f64, _v: f64, p: Point) -> Color {
        Color::ONE * self.noise.noise(p)
    }
}

impl Default for PerlinTexture {
    fn default() -> Self {
       PerlinTexture::new()
    }
}