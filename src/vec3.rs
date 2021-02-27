use std::ops;

#[derive(PartialEq, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn from(x: f64) -> Self {
        Vec3 { x, y: x, z: x }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn r(&self) -> f64 {
        self.x
    }
    pub fn g(&self) -> f64 {
        self.y
    }
    pub fn b(&self) -> f64 {
        self.z
    }

    pub fn write_color(&self) {
        let ir = (self.x * 255.0) as u8;
        let ig = (self.y * 255.0) as u8;
        let ib = (self.z * 255.0) as u8;

        println!("{} {} {}", ir, ig, ib);
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_creation() {
        let v1 = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(
            v1,
            Vec3 {
                x: 3.0,
                y: 4.0,
                z: 5.0
            }
        );

        let v2 = Vec3::zero();
        assert_eq!(
            v2,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );

        let v3 = Vec3::from(1.0);
        assert_eq!(
            v3,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn vector_add() {
        let v1 = Vec3::new(3.0, 4.0, 5.0);
        let v2 = Vec3::from(1.0);

        assert_eq!(v1 + v2, Vec3::new(4.0, 5.0, 6.0));
    }
}
