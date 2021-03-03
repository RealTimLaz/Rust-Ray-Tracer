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

        let unit_direction = ray.direction;
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, refraction_ratio)
        };

        Some((Ray::new(hit.p, direction), attenuation))
    }
}
