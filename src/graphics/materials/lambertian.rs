use crate::{
    graphics::{HitRecord, Ray},
    math::{Color, Vec3},
};

use super::Material;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let scattered_ray = Ray::new(hit.p, scatter_direction, ray.time);
        let attenuation = self.albedo;

        Some((scattered_ray, attenuation))
    }
}
