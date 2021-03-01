mod graphics;
mod math;

use graphics::{Hittable, Ray, Sphere};

use math::{Color, Point, Vec3};

fn ray_color(ray: Ray, world: &Vec<Box<dyn Hittable>>) -> Color {
    let potential_hit = world.hit(&ray, 0.0, f64::INFINITY);

    if let Some(h) = potential_hit {
        return 0.5 * (h.normal + Vec3::ONE);
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::ONE + t * Color::new(0.5, 0.7, 1.0)
}

fn render_image() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Point::new(0, 0, -1), 0.5)),
        Box::new(Sphere::new(Point::new(0, -100.5, 1), 100.0)),
    ];

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::ZERO;
    let horizontal = viewport_width * Vec3::RIGHT;
    let vertical = viewport_height * Vec3::UP;

    let lower_left_corner =
        &origin - &horizontal / 2.0 - &vertical / 2.0 - focal_length * Vec3::FORWARD;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let direction = &lower_left_corner + u * &horizontal + v * &vertical - &origin;
            let ray = Ray::new(&origin, &direction);
            let pixel = ray_color(ray, &world);
            pixel.write_color();
        }
    }

    eprint!("\nDone.\n");
}

fn main() {
    render_image();
}
