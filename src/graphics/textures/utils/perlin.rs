use rand::Rng;

use crate::math::{Point, Vec3};

const ARRAY_SIZE: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    permx: Vec<usize>,
    permy: Vec<usize>,
    permz: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let ranvec = (0..ARRAY_SIZE).map(|_| Vec3::random_unit_vector()).collect();
        let permx = Perlin::perlin_generate_perm();
        let permy = Perlin::perlin_generate_perm();
        let permz = Perlin::perlin_generate_perm();

        Perlin {
            ranvec,
            permx,
            permy,
            permz
        }
    }

    fn interpolate(c: Vec<Vec<Vec<Vec3>>>, u: f64, v: f64, w: f64) -> f64 {
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let mut accumulator = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accumulator += 
                    (i as f64 * u + (1.0 - i as f64) * (1.0 - u)) *
                    (j as f64 * v + (1.0 - j as f64) * (1.0 - v)) *
                    (k as f64 * w + (1.0 - k as f64) * (1.0 - w)) * 
                    c[i][j][k].dot(weight_v);
                }
            }
        }

        accumulator
    }

    pub fn turb(&self, p: Point, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2;
        }

        accum.abs()

    }

    pub fn noise(&self, p: Point) -> f64 {

        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = vec![vec![vec![Vec3::ZERO; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[
                        self.permx[((i + di as i32) & 255) as usize] ^ 
                        self.permy[((j + dj as i32) & 255) as usize] ^ 
                        self.permz[((k + dk as i32) & 255) as usize]]
                }
            }
        }

        Perlin::interpolate(c, u, v, w)
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