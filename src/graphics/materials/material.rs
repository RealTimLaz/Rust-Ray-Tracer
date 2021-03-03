use crate::{
    graphics::{HitRecord, Ray},
    math::Color,
};

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}
