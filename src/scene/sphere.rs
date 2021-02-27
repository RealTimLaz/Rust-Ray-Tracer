use super::{HitRecord, Hittable};
use crate::math::Point3;
use crate::render::{Material, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let root = (-half_b - sqrt_discriminant) / a;

        if root < t_max && root > t_min {
            let p = ray.at(root);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord::new(p, normal, root, &*self.material));
        }

        let root = (-half_b + sqrt_discriminant) / a;
        if root < t_max && root > t_min {
            let p = ray.at(root);
            let normal = (p - self.center) / self.radius;
            return Some(HitRecord::new(p, normal, root, &*self.material));
        }

        None
    }
}
