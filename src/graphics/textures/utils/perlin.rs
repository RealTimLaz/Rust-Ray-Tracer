use rand::Rng;

use crate::math::Point;

const ARRAY_SIZE: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    permx: Vec<usize>,
    permy: Vec<usize>,
    permz: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let ranfloat: Vec<f64> = (0..ARRAY_SIZE).map(|_| rng.gen::<f64>()).collect();
        let permx = Perlin::perlin_generate_perm();
        let permy = Perlin::perlin_generate_perm();
        let permz = Perlin::perlin_generate_perm();

        Perlin {
            ranfloat,
            permx,
            permy,
            permz
        }
    }

    pub fn noise(&self, p: Point) -> f64 {
        let i = (4.0 * p.x) as i32 & 255 ;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;

        self.ranfloat[self.permx[i as usize] ^ self.permy[j as usize] ^ self.permz[k as usize]]
    }

    fn perlin_generate_perm() -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut values: Vec<usize> = (0..ARRAY_SIZE).collect();

        for i in (1..ARRAY_SIZE).rev() {
            let target: usize = rng.gen_range(0..i);
            values.swap(i, target);
        }

        values
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Perlin::new()
    }
}