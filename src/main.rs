mod math;

use math::{Color, Hittable, Point3, Sphere};

use crate::math::Ray;
use crate::math::Vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    //World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point3::back(), 0.5)),
        Box::new(Sphere::new(Point3::new(0, -100.5, -1), 100.0)),
    ];

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::zero();
    let horizontal = Vec3::right() * viewport_width;
    let vertical = Vec3::up() * viewport_height;
    let lower_left_corner =
        origin - horizontal / 2 - vertical / 2 - (Vec3::forward() * focal_length);

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let color = ray_color(r, &world);
            color.write_color();
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
