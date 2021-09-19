use crate::graphics::{Aabb, HitRecord, Hittable, Ray};
use crate::math::{Point, Vec3};
use crate::graphics::materials::Material;

pub struct MovingSphere {
    pub center0: Point,
    pub center1: Point,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(center0: Point, center1: Point, time0: f64, time1: f64, radius: f64, material: Box<dyn Material>) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material
        }
    }

    pub fn center(&self, time: f64) -> Point {
         self.center0 + ((time - self.time0)/ (self.time1 - self.time0))*(self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
         // Quadratic formula
         let b = oc.dot(ray.direction);
         let c = oc.length_squared() - self.radius * self.radius;
 
         let discriminant = b.powi(2) - c;
 
         if discriminant < 0.0 {
             return None;
         }
 
         let sqrt_discriminant = discriminant.sqrt();
         let mut root = -b - sqrt_discriminant;
 
         if root < t_min || t_max < root {
             root = -b + sqrt_discriminant;
             if root < t_min || t_max < root {
                 return None;
             }
         }
         let intersection_point = ray.at(root);
         let normal = (intersection_point - self.center(ray.time)) / self.radius;
 
         Some(HitRecord::new(
             intersection_point,
             normal,
             root,
             ray,
             &*self.material,
         ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time0) - Vec3::ONE * self.radius, 
            self.center(time0) + Vec3::ONE * self.radius,
        );
        let box1 = Aabb::new(
            self.center(time1) - Vec3::ONE * self.radius, 
            self.center(time1) + Vec3::ONE * self.radius,
        );
        Some(box0.surrounding_box(&box1))
    }
}