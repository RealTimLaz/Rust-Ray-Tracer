use rand::Rng;
use ray_tracer::{graphics::{Bvh, Camera, Hittable, materials::{Dielectric, Lambertian, Metal}, models::{MovingSphere, Sphere}}, math::{Color, Point, Vec3}, render_image, utils::Config};

fn random_world() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![
        //Ground
        Box::new(Sphere::new(
            Point::new(0, -1000, 0),
            1000.0,
            Box::new(Lambertian::new_from_color(Color::new(0.5, 0.5, 0.5))),
        )),
    ];

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point::new(
                (a as f64) + 0.9 * rng.gen::<f64>(),
                0.2,
                (b as f64) + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point::new(3, 0.2, 0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random(0, 1);
                    let sphere_material = Lambertian::new_from_color(albedo);
                    let center2 = center + Vec3::UP * rng.gen_range(0.0..0.5);
                    world.push(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Box::new(sphere_material),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.5, 1);
                    let roughness = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, roughness);
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(sphere_material),
                    )));
                }
            }
        }
    }

    let material = Dielectric::new(1.5);
    world.push(Box::new(Sphere::new(
        Point::new(0, 1, 0),
        1.0,
        Box::new(material),
    )));

    let material = Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(
        Point::new(-4, 1, 0),
        1.0,
        Box::new(material),
    )));

    let material = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.push(Box::new(Sphere::new(
        Point::new(4, 1, 0),
        1.0,
        Box::new(material),
    )));

    world
}

fn setup_config() -> Config {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let camera = Camera::new(
        Point::new(13, 2, 3),
        Point::ZERO,
        Vec3::UP,
        20,
        aspect_ratio,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    let c = Config::new(camera, image_width, aspect_ratio);
    c.set_samples_per_pixel(100)
}

fn main() {
    let config = setup_config();
    let world = random_world();
    let world = Bvh::new(world, 0.0, 1.0);
    render_image(config, world);
}
