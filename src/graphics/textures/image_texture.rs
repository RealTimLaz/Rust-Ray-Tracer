use image::{DynamicImage, GenericImageView};

use crate::math::{Color, Point};

use super::{CheckerTexture, Texture};

enum SafeImage {
    Image(DynamicImage),
    None(CheckerTexture)
}

pub struct ImageTexture {
    image: SafeImage,
}

impl ImageTexture {
    pub fn new(filepath: &str) -> Self {
        let maybe_image = image::open(filepath);

        ImageTexture {
            image: match maybe_image {
                Ok(i) => SafeImage::Image(i),
                Err(e) => {
                    eprintln!("{:?}", e);
                    SafeImage::None(CheckerTexture::new_from_colors(Color::new(1, 0, 1), Color::new(0, 0, 0)))
                },
            }
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        match &self.image {
            SafeImage::None(c) => c.value(u, v, p),
            SafeImage::Image(img) => {
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);

                let (width, height) = img.dimensions();

                let mut i = (u * width as f64) as u32;
                let mut j = (v * height as f64) as u32;

                if i >= width {
                    i = width - 1;
                }

                if j >= height {
                    j = height - 1;
                }

                let pixel = img.get_pixel(i, j);
                
                Color::new(
                    pixel[0] as f64 / 255.0,
                    pixel[1] as f64 / 255.0,
                    pixel[2] as f64 / 255.0,
                )
            }
        }
    }
}