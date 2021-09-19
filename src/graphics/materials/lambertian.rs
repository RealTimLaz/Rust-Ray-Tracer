use crate::{graphics::{HitRecord, Ray, textures::{SolidColor, Texture}}, math::{Color, Vec3}};

use super::Material;

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Lambertian {

    pub fn new(texture: Box<dyn Texture>) -> Lambertian {
        Lambertian {
            albedo: texture
        }
    }

    pub fn new_from_color(albedo: Color) -> Lambertian {
        Lambertian { 
            albedo: Box::new(SolidColor::new(albedo)) 
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let scattered_ray = Ray::new(hit.p, scatter_direction, ray.time);
        let attenuation = self.albedo.value(hit.u, hit.v, hit.p);

        Some((scattered_ray, attenuation))
    }
}
