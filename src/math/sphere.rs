use super::{HitRecord, Hittable, Point3, Ray};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
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

        if root < t_min || t_max < root {
            let root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let mut hit_record = HitRecord::new(p, normal, t);
        let outward_normal = (p - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }
}
