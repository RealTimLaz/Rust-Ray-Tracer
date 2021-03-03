pub mod graphics;
pub mod math;
pub mod utils;

use graphics::{Hittable, Ray};

use math::Color;

use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;
use utils::Config;

fn ray_color<T: Hittable + Sync + Send>(ray: Ray, world: &T, depth: u32) -> Color {
    if depth == 0 {
        return Color::ZERO;
    }

    if let Some(hit) = world.hit(&ray, 0.001, f64::INFINITY) {
        match hit.material.scatter(&ray, &hit) {
            None => return Color::ZERO,
            Some((scattered_ray, attenuation)) => {
                return attenuation * ray_color(scattered_ray, world, depth - 1)
            }
        }
    }

    let unit_direction = ray.direction;
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
}

pub fn render_image<T: Hittable + Sync + Send>(config: Config, world: T) {
    let image_width = config.image_size.0;
    let image_height = config.image_size.1;

    let progress_bar = ProgressBar::new(image_height.into());

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let pixel_list = (0..(image_height * image_width))
        .into_par_iter()
        .map(|i| (i % image_width, image_height - i / image_width))
        .map(|(i, j)| {
            if i == 0 {
                progress_bar.inc(1);
            }
            // Random number generator
            let mut rng = rand::thread_rng();
            let mut pixel_color = Color::ZERO;
            for _ in 0..config.samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = ((j as f64) + rng.gen::<f64>()) / (image_height - 1) as f64;

                let ray = config.camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(ray, &world, config.max_depth);
            }
            (pixel_color, i, j)
        })
        .collect::<Vec<(Color, u32, u32)>>();

    for pixel in pixel_list {
        pixel.0.write_color(config.samples_per_pixel);
    }

    progress_bar.inc(1);
}
