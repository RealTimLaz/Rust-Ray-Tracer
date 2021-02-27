use std::fmt::{Debug, Formatter, Result};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new<T, U, V>(x: T, y: U, z: V) -> Self
    where
        T: Into<f64> + Copy,
        U: Into<f64> + Copy,
        V: Into<f64> + Copy,
    {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn from<T>(x: T) -> Self
    where
        T: Into<f64> + Copy,
    {
        Vec3 {
            x: x.into(),
            y: x.into(),
            z: x.into(),
        }
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

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> Mul<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Vec3 {
            x: self.x * rhs.into(),
            y: self.y * rhs.into(),
            z: self.z * rhs.into(),
        }
    }
}

impl<T> Div<T> for Vec3
where
    T: Into<f64> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Vec3 {
            x: self.x / rhs.into(),
            y: self.y / rhs.into(),
            z: self.z / rhs.into(),
        }
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Unable to index into Vec3 with index {}", i),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Unable to index into Vec3 with index {}", i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vector_creation() {
        let v1 = Vec3::new(3.0, 4, 5.0 as f32);
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

        let v3 = Vec3::from(1);
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
        let v1 = Vec3::new(3, 4, 5);
        let v2 = Vec3::from(1);

        assert_eq!(v1 + v2, Vec3::new(4, 5, 6));
    }

    #[test]
    fn vector_negate() {
        let v1 = -Vec3::new(3, 4, 5);
        assert_eq!(v1, Vec3::new(-3, -4, -5));
    }

    #[test]
    fn vector_sub() {
        let v1 = Vec3::new(3, 4, 5);
        let v2 = Vec3::from(1);

        assert_eq!(v1 - v2, Vec3::new(2, 3, 4));
    }

    #[test]
    fn vector_mul() {
        let v1 = Vec3::new(3, 4, 5);
        assert_eq!(v1 * 4, Vec3::new(12, 16, 20));
    }

    #[test]
    fn vector_div() {
        let v1 = Vec3::new(12, 16, 20);
        assert_eq!(v1 / 4, Vec3::new(3, 4, 5));
    }

    #[test]
    fn vector_index() {
        let v1 = Vec3::new(3, 4, 5);
        assert_eq!(v1[0], 3.0);
        assert_eq!(v1[1], 4.0);
        assert_eq!(v1[2], 5.0);

        let mut v1 = Vec3::new(3, 4, 5);
        v1[0] = 2.0;
        assert_eq!(v1[0], 2.0);
        assert_eq!(v1[1], 4.0);
        assert_eq!(v1[2], 5.0);
    }

    #[test]
    #[should_panic(expected = "Unable to index into Vec3 with index")]
    fn vector_index_panic() {
        let v1 = Vec3::new(3, 4, 5);
        v1[12];
    }
}
