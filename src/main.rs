use rand::Rng;
use ray_tracer::{graphics::{Bvh, Camera, Hittable, materials::{Dielectric, Lambertian, Metal}, models::{MovingSphere, Sphere}, textures::CheckerTexture}, math::{Color, Point, Vec3}, render_image, utils::Config};

fn random_world() -> Vec<Box<dyn Hittable>> {
    let checker = Box::new(CheckerTexture::new_from_colors(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let mut world: Vec<Box<dyn Hittable>> = vec![
        //Ground
        Box::new(Sphere::new(
            Point::new(0, -1000, 0),
            1000.0,
            Box::new(Lambertian::new(checker)),
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

fn two_spheres() -> Vec<Box<dyn Hittable>> {
    vec![
        Box::new(
            Sphere::new(
                Point::new(0, -10, 0),
                10.0,
                Box::new(Lambertian::new(Box::new(CheckerTexture::new_from_colors(Color::new(0.2, 0.3, 0.1), Color::new(0.9,0.9,0.9))))),
            )
        ),
        Box::new(
            Sphere::new(
                Point::new(0, 10, 0),
                10.0,
                Box::new(Lambertian::new(Box::new(CheckerTexture::new_from_colors(Color::new(0.2, 0.3, 0.1), Color::new(0.9,0.9,0.9))))),     
            )
        )
    ]
}

fn setup_config() -> Config{

    let scene_selector = 0;

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;

    let fov = 20.0;
    let mut aperture = 0.0;

    let world = match scene_selector {
        0 => {
           two_spheres()
        },

        _ => {
            aperture = 0.1;
            random_world()
        }
    };

    let camera = Camera::new(
        Point::new(13, 2, 3),
        Point::ZERO,
        Vec3::UP,
        fov,
        aspect_ratio,
        aperture,
        10.0,
        0.0,
        1.0,
    );

    let world = Bvh::new(world, 0.0, 1.0);

    let c = Config::new(Box::new(world), camera, image_width, aspect_ratio);
    c.set_samples_per_pixel(100)
}

fn main() {
    let config = setup_config();
    render_image(config);
}
