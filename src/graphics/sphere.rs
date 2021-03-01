use crate::math::Point;

use super::{HitRecord, Hittable, Ray};
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - &self.center;
        // Quadratic formula
        let a = ray.direction.length_squared();
        let b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-b - sqrt_discriminant) / a;

        if root < t_min || t_max < root {
            root = (-b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let intersection_point = ray.at(root);
        let normal = (&intersection_point - &self.center) / self.radius;

        Some(HitRecord::new(intersection_point, normal, root, ray))
    }
}
