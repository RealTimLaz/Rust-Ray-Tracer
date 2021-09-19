use std::ops::Index;

use crate::math::Vec3;
use crate::graphics::Ray;


#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub minimum: Vec3,
    pub maximum: Vec3,
}

impl Index<usize> for Aabb {
    type Output = Vec3;
    fn index(&self, i: usize) -> &Vec3 {
        match i {
            0 => &self.minimum,
            _ => &self.maximum,
        }
    }
}

impl Aabb {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Aabb {
            minimum,
            maximum,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut t0 = (self[ray.sign[0]].x - ray.origin.x) * ray.inv_direction.x;
        let mut t1 = (self[1 - ray.sign[0]].x - ray.origin.x) * ray.inv_direction.x;

        let ty0 = (self[ray.sign[1]].y - ray.origin.y) * ray.inv_direction.y;
        let ty1 = (self[1 - ray.sign[1]].y - ray.origin.y) * ray.inv_direction.y;

        if t0 > ty1 || ty0 > t1 {
            return false;
        }

        if ty0 > t0 {
            t0 = ty0;
        }
        if ty1 < t1 {
            t1 = ty1;
        }

        let tz0 = (self[ray.sign[2]].z - ray.origin.z) * ray.inv_direction.z;
        let tz1 = (self[1 - ray.sign[2]].z - ray.origin.z) * ray.inv_direction.z;

        if t0 > tz1 || tz0 > t1 {
            return false;
        }

        if tz0 > t0 {
            t0 = tz0;
        }
        if tz1 < t1 {
            t1 = tz1;
        }

        t0 < t_max && t1 > t_min
    }

    pub fn surrounding_box(&self, other: &Aabb) -> Aabb {
        let small = Vec3::new(
            self.minimum.x.min(other.minimum.x),
            self.minimum.y.min(other.minimum.y),
            self.minimum.z.min(other.minimum.z),
        );

        let big = Vec3::new(
            self.maximum.x.max(other.maximum.x),
            self.maximum.y.max(other.maximum.y),
            self.maximum.z.max(other.maximum.z),
        );

        Aabb::new(small, big)
    }
}