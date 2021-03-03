use crate::graphics::Camera;

pub struct Config {
    pub camera: Camera,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub image_size: (u32, u32),
}

impl Config {
    pub fn new(camera: Camera, image_width: u32, aspect_ratio: f64) -> Self {
        let image_height = ((image_width as f64) / aspect_ratio) as u32;
        Config {
            camera,
            samples_per_pixel: 100,
            max_depth: 50,
            image_size: (image_width, image_height),
        }
    }

    pub fn set_samples_per_pixel(self, samples_per_pixel: u32) -> Config {
        Config {
            samples_per_pixel,
            ..self
        }
    }

    pub fn set_max_depth(self, max_depth: u32) -> Config {
        Config { max_depth, ..self }
    }
}
