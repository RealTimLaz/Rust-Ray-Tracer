use super::{Point3, Ray, Vec3};

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
            front_face: false,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
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
