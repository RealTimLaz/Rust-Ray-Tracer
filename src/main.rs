mod math;
mod render;
mod scene;

use rand::Rng;

use math::{Color, Point3, Vec3};
use render::{Camera, Lambertian, Metal, Ray};
use scene::{Hittable, Sphere};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));

    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    //World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::back(), 0.5, Box::new(material_center))),
        Box::new(Sphere::new(
            Point3::new(0, -100.5, -1),
            100.0,
            Box::new(material_ground),
        )),
        Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Box::new(material_left),
        )),
        Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Box::new(material_right),
        )),
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
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);

                color = color + ray_color(r, &world, max_depth);
            }

            color.write_color(samples_per_pixel);
        }
    }

    eprint!("\nDone.\n");
}

fn ray_color(r: Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::zero();
    }

    let hit_record = world.hit(&r, 0.001, f64::MAX);

    if let Some(h) = hit_record {
        if let Some((scatter_ray, attenuation)) = h.material().scatter(&r, &h) {
            return attenuation * ray_color(scatter_ray, world, depth - 1);
        }

        return Color::zero();
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    Vec3::from(1) * (1.0 - t) + t * Vec3::new(0.5, 0.7, 1.0)
}
