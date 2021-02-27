use super::{Point3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at<T>(&self, t: T) -> Vec3
    where
        T: Into<f64> + Copy,
    {
        self.origin + (self.direction * t)
    }
}
