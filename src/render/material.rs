use crate::{
    math::{Color, Vec3},
    scene::HitRecord,
};

use super::Ray;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal() + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal();
        }
        Some((Ray::new(hit_record.p(), scatter_direction), self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Metal {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit_vector().reflect(hit_record.normal());
        let out_ray = Ray::new(
            hit_record.p(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );

        if out_ray.direction().dot(&hit_record.normal()) < 0.0 {
            return None;
        }

        Some((out_ray, self.albedo))
    }
}
