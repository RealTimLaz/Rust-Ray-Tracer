use crate::math::{Point, Vec3};

use super::{Aabb, Ray, materials::Material};

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point,
        outward_normal: Vec3,
        t: f64,
        ray: &Ray,
        material: &'a dyn Material,
    ) -> HitRecord<'a> {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        HitRecord {
            p,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut current_record = None;
        let mut closest_so_far = t_max;

        for object in self.iter() {
            let optional_hit_record = object.hit(ray, t_min, closest_so_far);

            match optional_hit_record {
                None => continue,
                Some(h) => {
                    closest_so_far = h.t;
                    current_record = Some(h);
                }
            }
        }

        current_record
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut output_box = Aabb::new(Vec3::ZERO, Vec3::ZERO);
        let mut first_box = true;

        for object in self.iter() {
            match object.bounding_box(time0, time1) {
                None => return None,
                Some (b) => {
                    output_box = if first_box {b} else {output_box.surrounding_box(&b)};
                    first_box = false;
                }
            }
        }

        Some(output_box)
    }   
}
