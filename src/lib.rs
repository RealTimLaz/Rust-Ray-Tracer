mod graphics;
mod math;

use graphics::models::Sphere;
use graphics::{Camera, Hittable, Ray};

use math::{Color, Point, Vec3};

use rand::Rng;

fn ray_color<T: Hittable>(ray: Ray, world: &T, depth: u32) -> Color {
    if depth == 0 {
        return Color::ZERO;
    }

    if let Some(hit) = world.hit(&ray, 0.001, f64::INFINITY) {
        let target = hit.p + hit.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color(Ray::new(hit.p, target - hit.p), world, depth - 1);
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

    // Random number generator
    let mut rng = rand::thread_rng();

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point::new(0, 0, -1), 0.5)),
        Box::new(Sphere::new(Point::new(0, -100.5, 1), 100.0)),
    ];

    // Camera
    let cam = Camera::new();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::ZERO;
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = ((j as f64) + rng.gen::<f64>()) / (image_height - 1) as f64;

                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(ray, &world, max_depth);
            }
            pixel_color.write_color(samples_per_pixel);
        }
    }

    eprint!("\nDone.\n");
}
