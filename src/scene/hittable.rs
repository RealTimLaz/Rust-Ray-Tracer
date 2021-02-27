use crate::math::{Point3, Vec3};
use crate::render::{Material, Ray};

pub struct HitRecord<'a> {
    p: Point3,
    normal: Vec3,
    t: f64,
    material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point3, normal: Vec3, t: f64, material: &dyn Material) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            material,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn p(&self) -> Point3 {
        self.p
    }

    pub fn material(&self) -> &dyn Material {
        self.material
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut result = None;

        for object in self.iter() {
            let hit_result = object.hit(ray, t_min, closest_so_far);
            match hit_result {
                None => continue,
                Some(hit_record) => {
                    closest_so_far = hit_record.t();
                    result = Some(hit_record);
                }
            }
        }

        result
    }
}
