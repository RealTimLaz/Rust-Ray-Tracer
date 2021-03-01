mod math;

use self::math::Color;

fn write_ppm() {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let pixel = Color::new(
                (i as f64) / (image_width - 1) as f64,
                (j as f64) / (image_height - 1) as f64,
                0.25,
            );
            pixel.write_color();
        }
    }

    eprint!("\nDone.\n");
}

fn main() {
    write_ppm();
}
