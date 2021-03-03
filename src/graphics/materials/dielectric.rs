use crate::{
    graphics::{HitRecord, Ray},
    math::Color,
};

use super::Material;

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::ONE;
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.normalize();
        let refracted = unit_direction.refract(hit.normal, refraction_ratio);

        Some((Ray::new(hit.p, refracted), attenuation))
    }
}
