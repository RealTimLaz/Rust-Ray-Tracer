mod vec3;

use self::vec3::Vec3;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", 255);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let pixel = Vec3::new(
                (i as f64) / (IMAGE_WIDTH - 1) as f64,
                (j as f64) / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );
            pixel.write_color();
        }
    }

    eprint!("\nDone\n");
}
