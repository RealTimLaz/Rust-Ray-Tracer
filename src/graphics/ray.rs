use crate::math::{Point, Vec3};
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
    pub time: f64,
    pub inv_direction: Vec3,
    pub sign: [usize; 3],
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3, time: f64) -> Self {
        let inv_direction = Vec3::new(
            1.0 / direction.x,
            1.0 / direction.y,
            1.0 / direction.z,
        );

        let sign = [
            (inv_direction.x < 0.0) as usize,
            (inv_direction.y < 0.0) as usize,
            (inv_direction.z < 0.0) as usize,
        ];

        Ray { 
            origin, 
            direction: direction.normalize(), 
            time,
            inv_direction,
            sign,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
