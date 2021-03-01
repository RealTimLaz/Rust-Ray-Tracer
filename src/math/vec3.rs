pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn write_color(self) {
        let ir = (255.999 * self.x) as u8;
        let ig = (255.999 * self.y) as u8;
        let ib = (255.999 * self.z) as u8;

        println!("{} {} {}", ir, ig, ib);
    }
}
