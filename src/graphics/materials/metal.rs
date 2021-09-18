use crate::{
    graphics::{HitRecord, Ray},
    math::{Color, Vec3},
};

use super::Material;

pub struct Metal {
    albedo: Color,
    roughness: f64,
}

impl Metal {
    pub fn new<T>(albedo: Color, roughness: T) -> Metal
    where
        T: Into<f64> + Copy,
    {
        Metal {
            albedo,
            roughness: roughness.into(),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction.reflect(hit.normal);
        if reflected.dot(hit.normal) <= 0.0 {
            return None;
        }
        let scattered_ray = Ray::new(
            hit.p,
            reflected + self.roughness * Vec3::random_in_unit_sphere(),
            ray.time,
        );
        let attenuation = self.albedo;
        Some((scattered_ray, attenuation))
    }
}
