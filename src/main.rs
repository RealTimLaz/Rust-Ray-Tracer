mod math;
mod render;
mod scene;

use rand::Rng;

use math::{Color, Point3, Vec3};
use render::{Camera, Ray};
use scene::{Hittable, Sphere};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    //World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::back(), 0.5)),
        Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100.0)),
    ];

    // Camera
    let cam = Camera::new();

    // Misc
    let mut rng = rand::thread_rng();

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color = Color::zero();
            for s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);

                color = color + ray_color(r, &world);
            }

            color.write_color(samples_per_pixel);
        }
    }

    eprint!("\nDone.\n");
}

fn ray_color(r: Ray, world: &dyn Hittable) -> Color {
    let hit_record = world.hit(&r, 0.0, f64::INFINITY);

    if let Some(h) = hit_record {
        return 0.5 * (h.normal() + Vec3::from(1));
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    Vec3::from(1) * (1.0 - t) + t * Vec3::new(0.5, 0.7, 1.0)
}
