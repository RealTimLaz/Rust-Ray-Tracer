mod graphics;
mod math;

use graphics::materials::{Dielectric, Lambertian, Metal};
use graphics::models::Sphere;
use graphics::{Camera, Hittable, Ray};

use math::{Color, Point};

use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;

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

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
}

pub fn render_image() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let progress_bar = ProgressBar::new(image_height.into());

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        //Ground
        Box::new(Sphere::new(
            Point::new(0, -100.5, -1),
            100.0,
            Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
        )),
        //Middle
        Box::new(Sphere::new(
            Point::new(0, 0, -1),
            0.5,
            Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
        )),
        //Left
        Box::new(Sphere::new(
            Point::new(-1, 0, -1),
            0.5,
            Box::new(Dielectric::new(1.5)),
        )),
        Box::new(Sphere::new(
            Point::new(1, 0, -1),
            0.5,
            Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0)),
        )),
    ];

    // Camera
    let cam = Camera::new();

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
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = ((j as f64) + rng.gen::<f64>()) / (image_height - 1) as f64;

                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(ray, &world, max_depth);
            }
            (pixel_color, i, j)
        })
        .collect::<Vec<(Color, u32, u32)>>();

    for pixel in pixel_list {
        pixel.0.write_color(samples_per_pixel);
    }

    progress_bar.inc(1);
}
