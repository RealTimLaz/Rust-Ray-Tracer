use crate::math::{Point, Vec3};
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3, time: f64) -> Self {
        Ray { 
            origin, 
            direction: direction.normalize(), 
            time 
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
